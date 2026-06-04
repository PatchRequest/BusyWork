use crate::categories::Categories;
use crate::tasks::{TaskDescriptor, TaskParams};
use rand::rngs::ThreadRng;
use std::hint::black_box;
use windows::Win32::Foundation::*;
use windows::Win32::System::Diagnostics::ToolHelp::*;
use windows::Win32::System::SystemInformation::*;
use windows::Win32::System::DataExchange::*;
use windows::Win32::UI::WindowsAndMessaging::*;

pub fn register() -> Vec<TaskDescriptor> {
    vec![
        TaskDescriptor {
            name: "enumerate_windows",
            category: Categories::WINAPI,
            func: enumerate_windows,
        },
        TaskDescriptor {
            name: "enumerate_processes",
            category: Categories::WINAPI,
            func: enumerate_processes,
        },
        TaskDescriptor {
            name: "query_system_info",
            category: Categories::WINAPI,
            func: query_system_info,
        },
        TaskDescriptor {
            name: "read_clipboard",
            category: Categories::WINAPI,
            func: read_clipboard,
        },
    ]
}

unsafe extern "system" fn enum_window_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let windows = &mut *(lparam.0 as *mut Vec<HWND>);
    windows.push(hwnd);
    BOOL(1)
}

fn enumerate_windows(params: &TaskParams, _rng: &mut ThreadRng) {
    unsafe {
        let mut windows: Vec<HWND> = Vec::new();
        let ptr = &mut windows as *mut Vec<HWND> as isize;
        let _ = EnumWindows(Some(enum_window_callback), LPARAM(ptr));

        for hwnd in windows.iter().take(params.iterations) {
            let mut text = [0u16; 256];
            GetWindowTextW(*hwnd, &mut text);
            black_box(&text);
        }
    }
}

fn enumerate_processes(params: &TaskParams, _rng: &mut ThreadRng) {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        let snapshot = match snapshot {
            Ok(h) => h,
            Err(_) => return,
        };

        let mut entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        if Process32FirstW(snapshot, &mut entry).is_ok() {
            for _ in 0..params.iterations {
                black_box(entry.th32ProcessID);
                black_box(&entry.szExeFile);
                if Process32NextW(snapshot, &mut entry).is_err() {
                    break;
                }
            }
        }

        let _ = CloseHandle(snapshot);
    }
}

fn query_system_info(params: &TaskParams, _rng: &mut ThreadRng) {
    unsafe {
        for _ in 0..params.call_depth {
            let mut sys_info = SYSTEM_INFO::default();
            GetSystemInfo(&mut sys_info);
            black_box(sys_info.dwNumberOfProcessors);

            let mut mem_status = MEMORYSTATUSEX {
                dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
                ..Default::default()
            };
            let _ = GlobalMemoryStatusEx(&mut mem_status);
            black_box(mem_status.ullTotalPhys);
        }
    }
}

fn read_clipboard(params: &TaskParams, _rng: &mut ThreadRng) {
    unsafe {
        for _ in 0..params.call_depth {
            if OpenClipboard(HWND::default()).is_ok() {
                let _ = black_box(GetClipboardData(1)); // CF_TEXT
                let _ = CloseClipboard();
            }
        }
    }
}
