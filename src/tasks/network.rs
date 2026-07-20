use crate::categories::Categories;
use crate::tasks::{ScratchBuffer, TaskDescriptor, TaskParams};
use crate::workdata::WorkData;
use rand::rngs::ThreadRng;
use rand::RngCore;
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
        TaskDescriptor {
            name: "http_head_request",
            category: Categories::NETWORK,
            func: http_head_request,
        },
        TaskDescriptor {
            name: "tcp_connect_probe",
            category: Categories::NETWORK,
            func: tcp_connect_probe,
        },
        TaskDescriptor {
            name: "dns_varied_ports",
            category: Categories::NETWORK,
            func: dns_varied_ports,
        },
        TaskDescriptor {
            name: "http_post_discard",
            category: Categories::NETWORK,
            func: http_post_discard,
        },
    ]
}

fn dns_lookups(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData, scratch: &mut ScratchBuffer) {
    let hosts = [
        "google.com:80",
        "microsoft.com:80",
        "cloudflare.com:80",
        "github.com:80",
        "amazon.com:80",
        "apple.com:80",
        "mozilla.org:80",
        "wikipedia.org:80",
        "reddit.com:80",
        "stackoverflow.com:80",
        "twitter.com:80",
        "linkedin.com:80",
        "facebook.com:80",
        "youtube.com:80",
        "netflix.com:80",
        "twitch.tv:80",
        "spotify.com:80",
        "dropbox.com:80",
        "zoom.us:80",
        "slack.com:80",
        "discord.com:80",
        "akamai.com:80",
        "fastly.com:80",
        "aws.amazon.com:80",
    ];
    let start_idx = work.derive_usize(0) % hosts.len();
    for i in 0..params.io_rounds() {
        let host = hosts[(start_idx + i) % hosts.len()];
        let _ = host.to_socket_addrs().map(|addrs| {
            for addr in addrs {
                let bytes = match addr {
                    std::net::SocketAddr::V4(v4) => v4.ip().octets().to_vec(),
                    std::net::SocketAddr::V6(v6) => v6.ip().octets().to_vec(),
                };
                scratch.absorb(&bytes);
                black_box(addr);
            }
        });
    }
}

fn http_get(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData, scratch: &mut ScratchBuffer) {
    let targets: &[(&str, u16, &str)] = &[
        ("httpbin.org", 80, "/get"),
        ("ip-api.com", 80, "/json"),
        ("ifconfig.me", 80, "/ip"),
        ("api.ipify.org", 80, "/"),
        ("httpbin.org", 80, "/headers"),
        ("httpbin.org", 80, "/user-agent"),
        ("worldtimeapi.org", 80, "/api/ip"),
        ("httpbin.org", 80, "/ip"),
        ("icanhazip.com", 80, "/"),
        ("checkip.amazonaws.com", 80, "/"),
        ("api.myip.com", 80, "/"),
    ];
    let start_idx = work.derive_usize(0) % targets.len();
    for i in 0..params.call_depth.max(1) {
        let &(host, port, path) = &targets[(start_idx + i) % targets.len()];
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
        work.blend_into(&mut response);
        scratch.blend_into(&mut response);
        scratch.absorb(&response[..response.len().min(256)]);
        black_box(&response);
    }
}

fn ntp_query(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData, scratch: &mut ScratchBuffer) {
    let servers = [
        "pool.ntp.org:123",
        "time.google.com:123",
        "time.cloudflare.com:123",
        "time.windows.com:123",
        "time.apple.com:123",
        "time.nist.gov:123",
        "ntp.ubuntu.com:123",
    ];
    let start_idx = work.derive_usize(0) % servers.len();
    for i in 0..params.call_depth.max(1) {
        let server = servers[(start_idx + i) % servers.len()];
        let socket = match UdpSocket::bind("0.0.0.0:0") {
            Ok(s) => s,
            Err(_) => continue,
        };
        set_socket_timeouts(&socket, 3000);
        let mut packet = [0u8; 48];
        packet[0] = 0x1B;
        if socket.send_to(&packet, server).is_err() {
            continue;
        }
        let mut response = [0u8; 48];
        let _ = socket.recv_from(&mut response);
        scratch.absorb(&response);
        black_box(&response);
    }
}

fn http_head_request(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData, scratch: &mut ScratchBuffer) {
    let targets: &[(&str, u16, &str)] = &[
        ("httpbin.org", 80, "/get"),
        ("httpbin.org", 80, "/headers"),
        ("httpbin.org", 80, "/ip"),
        ("httpbin.org", 80, "/user-agent"),
        ("ip-api.com", 80, "/json"),
        ("ifconfig.me", 80, "/ip"),
        ("worldtimeapi.org", 80, "/api/ip"),
        ("checkip.amazonaws.com", 80, "/"),
        ("icanhazip.com", 80, "/"),
        ("api.ipify.org", 80, "/"),
    ];
    let start_idx = work.derive_usize(0) % targets.len();
    for i in 0..params.call_depth.max(1) {
        let &(host, port, path) = &targets[(start_idx + i) % targets.len()];
        let addr = format!("{}:{}", host, port);
        let stream = match TcpStream::connect(&*addr) {
            Ok(s) => s,
            Err(_) => continue,
        };
        set_socket_timeouts(&stream, 3000);
        let request = format!(
            "HEAD {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            path, host
        );
        let mut stream = stream;
        if stream.write_all(request.as_bytes()).is_err() {
            continue;
        }
        let mut response = vec![0u8; 2048];
        let _ = stream.read(&mut response);
        scratch.absorb(&response[..response.len().min(256)]);
        black_box(&response);
    }
}

fn tcp_connect_probe(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData, scratch: &mut ScratchBuffer) {
    let targets: &[(&str, u16)] = &[
        ("google.com", 80),
        ("google.com", 443),
        ("cloudflare.com", 80),
        ("cloudflare.com", 443),
        ("github.com", 443),
        ("microsoft.com", 80),
        ("microsoft.com", 443),
        ("amazon.com", 443),
        ("1.1.1.1", 53),
        ("8.8.8.8", 53),
    ];
    let start_idx = work.derive_usize(0) % targets.len();
    for i in 0..params.io_rounds() {
        let &(host, port) = &targets[(start_idx + i) % targets.len()];
        let addr = format!("{}:{}", host, port);
        if let Ok(stream) = TcpStream::connect(&*addr) {
            set_socket_timeouts(&stream, 2000);
            let marker = [host.as_bytes()[0], port as u8];
            scratch.absorb(&marker);
            black_box(&stream);
        }
    }
}

fn dns_varied_ports(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData, scratch: &mut ScratchBuffer) {
    let hosts = [
        "example.com:21",
        "example.com:22",
        "example.com:25",
        "example.com:53",
        "example.com:110",
        "example.com:143",
        "example.com:443",
        "example.com:993",
        "example.com:8080",
        "example.com:8443",
        "example.org:80",
        "example.org:443",
        "example.net:80",
        "example.net:25",
        "example.net:8080",
    ];
    let start_idx = work.derive_usize(0) % hosts.len();
    for i in 0..params.io_rounds() {
        let host = hosts[(start_idx + i) % hosts.len()];
        let _ = host.to_socket_addrs().map(|addrs| {
            for addr in addrs {
                let bytes = match addr {
                    std::net::SocketAddr::V4(v4) => v4.ip().octets().to_vec(),
                    std::net::SocketAddr::V6(v6) => v6.ip().octets().to_vec(),
                };
                scratch.absorb(&bytes);
                black_box(addr);
            }
        });
    }
}

fn http_post_discard(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData, scratch: &mut ScratchBuffer) {
    let targets: &[(&str, u16, &str, &str)] = &[
        ("httpbin.org", 80, "/post", "POST"),
        ("httpbin.org", 80, "/anything", "POST"),
        ("httpbin.org", 80, "/put", "PUT"),
        ("httpbin.org", 80, "/anything", "PUT"),
        ("httpbin.org", 80, "/patch", "PATCH"),
    ];

    let body_size = params.buffer_size.min(1024);
    let mut body = vec![0u8; body_size];
    rng.fill_bytes(&mut body);
    work.blend_into(&mut body);
    scratch.blend_into(&mut body);
    let body_hex: String = body.iter().map(|b| format!("{:02x}", b)).collect();

    let start_idx = work.derive_usize(0) % targets.len();
    for i in 0..params.call_depth.max(1) {
        let &(host, port, path, method) = &targets[(start_idx + i) % targets.len()];
        let addr = format!("{}:{}", host, port);
        let stream = match TcpStream::connect(&*addr) {
            Ok(s) => s,
            Err(_) => continue,
        };
        set_socket_timeouts(&stream, 3000);
        let request = format!(
            "{} {} HTTP/1.1\r\nHost: {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            method, path, host, body_hex.len(), body_hex
        );
        let mut stream = stream;
        if stream.write_all(request.as_bytes()).is_err() {
            continue;
        }
        let mut response = vec![0u8; 4096];
        let _ = stream.read(&mut response);
        scratch.absorb(&response[..response.len().min(256)]);
        black_box(&response);
    }
}
