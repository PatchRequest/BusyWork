use crate::categories::Categories;
use crate::tasks::{TaskDescriptor, TaskParams};
use crate::workdata::WorkData;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::hint::black_box;
use windows::core::w;
use windows::core::PCWSTR;
use windows::Win32::Foundation::*;
use windows::Win32::Storage::FileSystem::*;
use windows::Win32::System::DataExchange::*;
use windows::Win32::System::Diagnostics::ToolHelp::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::System::Memory::*;
use windows::Win32::System::SystemInformation::*;
use windows::Win32::System::Threading::*;
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
        TaskDescriptor {
            name: "get_system_metrics",
            category: Categories::WINAPI,
            func: get_system_metrics,
        },
        TaskDescriptor {
            name: "get_foreground_window_info",
            category: Categories::WINAPI,
            func: get_foreground_window_info,
        },
        TaskDescriptor {
            name: "get_cursor_position",
            category: Categories::WINAPI,
            func: get_cursor_position,
        },
        TaskDescriptor {
            name: "get_desktop_window_info",
            category: Categories::WINAPI,
            func: get_desktop_window_info,
        },
        TaskDescriptor {
            name: "get_logical_drives_info",
            category: Categories::WINAPI,
            func: get_logical_drives_info,
        },
        TaskDescriptor {
            name: "get_volume_info",
            category: Categories::WINAPI,
            func: get_volume_info,
        },
        TaskDescriptor {
            name: "get_disk_free_space",
            category: Categories::WINAPI,
            func: get_disk_free_space,
        },
        TaskDescriptor {
            name: "find_files_pattern",
            category: Categories::WINAPI,
            func: find_files_pattern,
        },
        TaskDescriptor {
            name: "get_module_handles",
            category: Categories::WINAPI,
            func: get_module_handles,
        },
        TaskDescriptor {
            name: "virtual_query_memory",
            category: Categories::WINAPI,
            func: virtual_query_memory,
        },
        TaskDescriptor {
            name: "get_system_directories",
            category: Categories::WINAPI,
            func: get_system_directories,
        },
        TaskDescriptor {
            name: "get_process_thread_ids",
            category: Categories::WINAPI,
            func: get_process_thread_ids,
        },
    ]
}

unsafe extern "system" fn enum_window_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let windows = &mut *(lparam.0 as *mut Vec<HWND>);
    windows.push(hwnd);
    BOOL(1)
}

fn enumerate_windows(params: &TaskParams, _rng: &mut ThreadRng, _work: &WorkData) {
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

fn enumerate_processes(params: &TaskParams, _rng: &mut ThreadRng, _work: &WorkData) {
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

fn query_system_info(params: &TaskParams, _rng: &mut ThreadRng, _work: &WorkData) {
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

fn read_clipboard(params: &TaskParams, _rng: &mut ThreadRng, _work: &WorkData) {
    unsafe {
        for _ in 0..params.call_depth {
            if OpenClipboard(HWND::default()).is_ok() {
                let _ = black_box(GetClipboardData(1));
                let _ = CloseClipboard();
            }
        }
    }
}

fn get_system_metrics(params: &TaskParams, rng: &mut ThreadRng, _work: &WorkData) {
    const METRIC_INDICES: [i32; 10] = [
        0,  // SM_CXSCREEN
        1,  // SM_CYSCREEN
        2,  // SM_CXVSCROLL
        3,  // SM_CYHSCROLL
        4,  // SM_CYCAPTION
        43, // SM_CMOUSEBUTTONS
        80, // SM_CMONITORS
        59, // SM_CXMAXTRACK
        60, // SM_CYMAXTRACK
        67, // SM_CLEANBOOT
    ];

    unsafe {
        for _ in 0..params.iterations {
            let idx = rng.gen_range(0..METRIC_INDICES.len());
            let metric = METRIC_INDICES[idx];
            let value = GetSystemMetrics(SYSTEM_METRICS_INDEX(metric));
            black_box(value);
        }
    }
}

fn get_foreground_window_info(params: &TaskParams, _rng: &mut ThreadRng, _work: &WorkData) {
    unsafe {
        for _ in 0..params.call_depth {
            let hwnd = GetForegroundWindow();
            if hwnd.0 == std::ptr::null_mut() {
                continue;
            }
            let mut text = [0u16; 256];
            GetWindowTextW(hwnd, &mut text);
            black_box(&text);

            let mut rect = RECT::default();
            let _ = GetWindowRect(hwnd, &mut rect);
            black_box(rect.left);
            black_box(rect.top);
            black_box(rect.right);
            black_box(rect.bottom);
        }
    }
}

fn get_cursor_position(params: &TaskParams, _rng: &mut ThreadRng, _work: &WorkData) {
    unsafe {
        for _ in 0..params.iterations {
            let mut point = POINT::default();
            let _ = GetCursorPos(&mut point);
            black_box(point.x);
            black_box(point.y);
        }
    }
}

fn get_desktop_window_info(params: &TaskParams, _rng: &mut ThreadRng, _work: &WorkData) {
    unsafe {
        for _ in 0..params.call_depth {
            let hwnd = GetDesktopWindow();

            let mut rect = RECT::default();
            let _ = GetWindowRect(hwnd, &mut rect);
            black_box(rect.left);
            black_box(rect.top);
            black_box(rect.right);
            black_box(rect.bottom);

            let mut client_rect = RECT::default();
            let _ = GetClientRect(hwnd, &mut client_rect);
            black_box(client_rect.right);
            black_box(client_rect.bottom);

            let mut text = [0u16; 256];
            GetWindowTextW(hwnd, &mut text);
            black_box(&text);
        }
    }
}

fn get_logical_drives_info(params: &TaskParams, _rng: &mut ThreadRng, _work: &WorkData) {
    unsafe {
        let mask = GetLogicalDrives();
        if mask == 0 {
            return;
        }
        black_box(mask);

        for bit in 0..26u32 {
            if mask & (1 << bit) == 0 {
                continue;
            }
            let letter = (b'A' + bit as u8) as u16;
            let path: [u16; 4] = [letter, ':' as u16, '\\' as u16, 0];
            let drive_type = GetDriveTypeW(PCWSTR(path.as_ptr()));
            black_box(drive_type);
        }

        for _ in 1..params.call_depth {
            let m = GetLogicalDrives();
            black_box(m);
        }
    }
}

fn get_volume_info(params: &TaskParams, _rng: &mut ThreadRng, _work: &WorkData) {
    unsafe {
        for _ in 0..params.call_depth {
            let mut volume_name = [0u16; 260];
            let mut serial_number: u32 = 0;
            let mut max_component_length: u32 = 0;
            let mut fs_flags: u32 = 0;
            let mut fs_name = [0u16; 260];

            let ok = GetVolumeInformationW(
                w!("C:\\"),
                Some(&mut volume_name),
                Some(&mut serial_number),
                Some(&mut max_component_length),
                Some(&mut fs_flags),
                Some(&mut fs_name),
            );
            if ok.is_ok() {
                black_box(&volume_name);
                black_box(serial_number);
                black_box(max_component_length);
                black_box(fs_flags);
                black_box(&fs_name);
            }
        }
    }
}

fn get_disk_free_space(params: &TaskParams, _rng: &mut ThreadRng, _work: &WorkData) {
    unsafe {
        for _ in 0..params.call_depth {
            let mut free_bytes_available: u64 = 0;
            let mut total_bytes: u64 = 0;
            let mut total_free_bytes: u64 = 0;

            let ok = GetDiskFreeSpaceExW(
                w!("C:\\"),
                Some(&mut free_bytes_available as *mut u64),
                Some(&mut total_bytes as *mut u64),
                Some(&mut total_free_bytes as *mut u64),
            );
            if ok.is_ok() {
                black_box(free_bytes_available);
                black_box(total_bytes);
                black_box(total_free_bytes);
            }
        }
    }
}

fn find_files_pattern(params: &TaskParams, _rng: &mut ThreadRng, _work: &WorkData) {
    unsafe {
        let mut find_data = WIN32_FIND_DATAW::default();
        let handle = FindFirstFileW(w!("C:\\Windows\\System32\\*.dll"), &mut find_data);
        let handle = match handle {
            Ok(h) => h,
            Err(_) => return,
        };

        black_box(&find_data.cFileName);
        black_box(find_data.nFileSizeHigh);
        black_box(find_data.nFileSizeLow);

        for _ in 1..params.iterations {
            if FindNextFileW(handle, &mut find_data).is_err() {
                break;
            }
            black_box(&find_data.cFileName);
            black_box(find_data.nFileSizeHigh);
            black_box(find_data.nFileSizeLow);
        }

        let _ = FindClose(handle);
    }
}

fn get_module_handles(params: &TaskParams, rng: &mut ThreadRng, _work: &WorkData) {
    let dll_names: &[PCWSTR] = &[
        w!("kernel32.dll"),
        w!("ntdll.dll"),
        w!("user32.dll"),
        w!("advapi32.dll"),
        w!("ws2_32.dll"),
        w!("shell32.dll"),
        w!("ole32.dll"),
        w!("crypt32.dll"),
        w!("bcrypt.dll"),
        w!("msvcrt.dll"),
        w!("gdi32.dll"),
        w!("combase.dll"),
    ];

    unsafe {
        for _ in 0..params.iterations {
            let idx = rng.gen_range(0..dll_names.len());
            let result = GetModuleHandleW(dll_names[idx]);
            match result {
                Ok(h) => {
                    black_box(h);
                }
                Err(_) => {
                    black_box(0u32);
                }
            }
        }
    }
}

fn virtual_query_memory(params: &TaskParams, _rng: &mut ThreadRng, _work: &WorkData) {
    unsafe {
        let mut addr: usize = 0;
        let info_size = std::mem::size_of::<MEMORY_BASIC_INFORMATION>();

        for _ in 0..params.iterations {
            let mut info = MEMORY_BASIC_INFORMATION::default();
            let result = VirtualQuery(
                Some(addr as *const std::ffi::c_void),
                &mut info,
                info_size,
            );
            if result == 0 {
                break;
            }
            black_box(info.BaseAddress);
            black_box(info.RegionSize);
            black_box(info.State);
            black_box(info.Type);

            let region_size = if info.RegionSize == 0 {
                4096
            } else {
                info.RegionSize
            };
            addr = match addr.checked_add(region_size) {
                Some(a) => a,
                None => break,
            };
        }
    }
}

fn get_system_directories(params: &TaskParams, _rng: &mut ThreadRng, _work: &WorkData) {
    unsafe {
        for _ in 0..params.call_depth {
            let mut sys_dir = [0u16; 260];
            let len = GetSystemDirectoryW(Some(&mut sys_dir));
            if len > 0 {
                black_box(&sys_dir[..len as usize]);
            }

            let mut win_dir = [0u16; 260];
            let len = GetWindowsDirectoryW(Some(&mut win_dir));
            if len > 0 {
                black_box(&win_dir[..len as usize]);
            }
        }
    }
}

fn get_process_thread_ids(params: &TaskParams, _rng: &mut ThreadRng, _work: &WorkData) {
    unsafe {
        for _ in 0..params.iterations {
            let pid = GetCurrentProcessId();
            black_box(pid);

            let tid = GetCurrentThreadId();
            black_box(tid);
        }
    }
}
