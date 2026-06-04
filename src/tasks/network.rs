use crate::categories::Categories;
use crate::tasks::{TaskDescriptor, TaskParams};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::hint::black_box;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs, UdpSocket};
use std::os::windows::io::AsRawSocket;

#[link(name = "ws2_32")]
extern "system" {
    fn setsockopt(s: usize, level: i32, optname: i32, optval: *const u8, optlen: i32) -> i32;
}

const SOL_SOCKET: i32 = 0xFFFF;
const SO_RCVTIMEO: i32 = 0x1006;
const SO_SNDTIMEO: i32 = 0x1005;

fn set_socket_timeouts(socket: &impl AsRawSocket, ms: u32) {
    let raw = socket.as_raw_socket() as usize;
    let val = ms.to_ne_bytes();
    unsafe {
        setsockopt(raw, SOL_SOCKET, SO_RCVTIMEO, val.as_ptr(), 4);
        setsockopt(raw, SOL_SOCKET, SO_SNDTIMEO, val.as_ptr(), 4);
    }
}

pub fn register() -> Vec<TaskDescriptor> {
    vec![
        TaskDescriptor {
            name: "dns_lookups",
            category: Categories::NETWORK,
            func: dns_lookups,
        },
        TaskDescriptor {
            name: "http_get",
            category: Categories::NETWORK,
            func: http_get,
        },
        TaskDescriptor {
            name: "ntp_query",
            category: Categories::NETWORK,
            func: ntp_query,
        },
    ]
}

fn dns_lookups(params: &TaskParams, rng: &mut ThreadRng) {
    let hosts = [
        "google.com:80",
        "microsoft.com:80",
        "cloudflare.com:80",
        "github.com:80",
        "amazon.com:80",
        "apple.com:80",
        "mozilla.org:80",
        "wikipedia.org:80",
    ];
    for _ in 0..params.iterations.min(50) {
        if let Some(host) = hosts.choose(rng) {
            let _ = host.to_socket_addrs().map(|addrs| {
                for addr in addrs {
                    black_box(addr);
                }
            });
        }
    }
}

fn http_get(params: &TaskParams, rng: &mut ThreadRng) {
    let targets: &[(&str, u16, &str)] = &[
        ("httpbin.org", 80, "/get"),
        ("ip-api.com", 80, "/json"),
        ("ifconfig.me", 80, "/ip"),
    ];
    for _ in 0..params.call_depth.min(3) {
        if let Some(&(host, port, path)) = targets.choose(rng) {
            let addr = format!("{}:{}", host, port);
            let stream = match TcpStream::connect(&*addr) {
                Ok(s) => s,
                Err(_) => continue,
            };
            set_socket_timeouts(&stream, 3000);
            let request = format!(
                "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
                path, host
            );
            let mut stream = stream;
            if stream.write_all(request.as_bytes()).is_err() {
                continue;
            }
            let mut response = vec![0u8; 4096];
            let _ = stream.read(&mut response);
            black_box(&response);
        }
    }
}

fn ntp_query(params: &TaskParams, _rng: &mut ThreadRng) {
    for _ in 0..params.call_depth.min(3) {
        let socket = match UdpSocket::bind("0.0.0.0:0") {
            Ok(s) => s,
            Err(_) => continue,
        };
        set_socket_timeouts(&socket, 3000);
        let mut packet = [0u8; 48];
        packet[0] = 0x1B; // NTP v3, client mode
        if socket.send_to(&packet, "pool.ntp.org:123").is_err() {
            continue;
        }
        let mut response = [0u8; 48];
        let _ = socket.recv_from(&mut response);
        black_box(&response);
    }
}
