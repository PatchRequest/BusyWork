use crate::categories::Categories;
use crate::tasks::{ScratchBuffer, TaskDescriptor, TaskParams};
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

fn enumerate_windows(
    params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let skip = work.derive_usize(0) % 8;
    unsafe {
        let mut windows: Vec<HWND> = Vec::new();
        let ptr = &mut windows as *mut Vec<HWND> as isize;
        let _ = EnumWindows(Some(enum_window_callback), LPARAM(ptr));

        let mut last_text = [0u16; 256];
        for hwnd in windows.iter().skip(skip).take(params.iterations) {
            let mut text = [0u16; 256];
            GetWindowTextW(*hwnd, &mut text);
            last_text = text;
            black_box(&text);
        }
        let text_bytes: &[u8] = std::slice::from_raw_parts(
            last_text.as_ptr() as *const u8,
            last_text.len() * 2,
        );
        scratch.absorb(text_bytes);
        black_box(&last_text);
    }
}

fn enumerate_processes(
    params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let skip = work.derive_usize(0) % 4;
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

        let mut last_pid: u32 = 0;
        if Process32FirstW(snapshot, &mut entry).is_ok() {
            let mut skipped = 0;
            loop {
                if skipped < skip {
                    skipped += 1;
                    if Process32NextW(snapshot, &mut entry).is_err() {
                        break;
                    }
                    continue;
                }
                break;
            }

            for _ in 0..params.iterations {
                last_pid = entry.th32ProcessID;
                black_box(entry.th32ProcessID);
                black_box(&entry.szExeFile);
                if Process32NextW(snapshot, &mut entry).is_err() {
                    break;
                }
            }
        }

        scratch.absorb(&last_pid.to_ne_bytes());
        black_box(last_pid);
        let _ = CloseHandle(snapshot);
    }
}

fn query_system_info(
    _params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let loops = _params.call_depth + (work.blend_seed() % 2) as usize;
    unsafe {
        for _ in 0..loops {
            let mut sys_info = SYSTEM_INFO::default();
            GetSystemInfo(&mut sys_info);
            black_box(sys_info.dwNumberOfProcessors);

            let mut mem_status = MEMORYSTATUSEX {
                dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
                ..Default::default()
            };
            let _ = GlobalMemoryStatusEx(&mut mem_status);
            black_box(mem_status.ullTotalPhys);

            scratch.absorb(&sys_info.dwNumberOfProcessors.to_ne_bytes());
            scratch.absorb(&mem_status.ullTotalPhys.to_ne_bytes());
        }
    }
}

fn read_clipboard(
    _params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let loops = _params.call_depth + (work.blend_seed() % 2) as usize;
    unsafe {
        for _ in 0..loops {
            if OpenClipboard(HWND::default()).is_ok() {
                let _ = black_box(GetClipboardData(1));
                let _ = CloseClipboard();
            }
            scratch.absorb(&[0xCB]);
        }
    }
}

fn get_system_metrics(
    params: &TaskParams,
    rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
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

    let start_idx = work.derive_usize(0) % 10;
    unsafe {
        for _ in 0..params.iterations {
            let idx = (start_idx + rng.gen_range(0..METRIC_INDICES.len())) % METRIC_INDICES.len();
            let metric = METRIC_INDICES[idx];
            let value = GetSystemMetrics(SYSTEM_METRICS_INDEX(metric));
            scratch.absorb(&value.to_ne_bytes());
            black_box(value);
        }
    }
}

fn get_foreground_window_info(
    _params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let loops = _params.call_depth + (work.blend_seed() % 2) as usize;
    unsafe {
        for _ in 0..loops {
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

            let rect_bytes: &[u8] = std::slice::from_raw_parts(
                &rect as *const RECT as *const u8,
                std::mem::size_of::<RECT>(),
            );
            scratch.absorb(rect_bytes);
        }
    }
}

fn get_cursor_position(
    params: &TaskParams,
    _rng: &mut ThreadRng,
    _work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let mut last_point = POINT::default();
    unsafe {
        for _ in 0..params.iterations {
            let mut point = POINT::default();
            let _ = GetCursorPos(&mut point);
            last_point = point;
            black_box(point.x);
            black_box(point.y);
        }
        let point_bytes: &[u8] = std::slice::from_raw_parts(
            &last_point as *const POINT as *const u8,
            std::mem::size_of::<POINT>(),
        );
        scratch.absorb(point_bytes);
        black_box(&last_point);
    }
}

fn get_desktop_window_info(
    _params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let loops = _params.call_depth + (work.blend_seed() % 2) as usize;
    unsafe {
        for _ in 0..loops {
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

            let rect_bytes: &[u8] = std::slice::from_raw_parts(
                &rect as *const RECT as *const u8,
                std::mem::size_of::<RECT>(),
            );
            scratch.absorb(rect_bytes);
        }
    }
}

fn get_logical_drives_info(
    _params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let extra_loops = _params.call_depth + (work.blend_seed() % 2) as usize;
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

        for _ in 1..extra_loops {
            let m = GetLogicalDrives();
            black_box(m);
        }

        scratch.absorb(&mask.to_ne_bytes());
        black_box(mask);
    }
}

fn get_volume_info(
    _params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let loops = _params.call_depth + (work.blend_seed() % 2) as usize;
    unsafe {
        for _ in 0..loops {
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
                scratch.absorb(&serial_number.to_ne_bytes());
            }
        }
    }
}

fn get_disk_free_space(
    _params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let loops = _params.call_depth + (work.blend_seed() % 2) as usize;
    unsafe {
        for _ in 0..loops {
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
                scratch.absorb(&total_free_bytes.to_ne_bytes());
            }
        }
    }
}

fn find_files_pattern(
    params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let skip = work.derive_usize(0) % 8;
    unsafe {
        let mut find_data = WIN32_FIND_DATAW::default();
        let handle = FindFirstFileW(w!("C:\\Windows\\System32\\*.dll"), &mut find_data);
        let handle = match handle {
            Ok(h) => h,
            Err(_) => return,
        };

        // Skip first N entries
        for _ in 0..skip {
            if FindNextFileW(handle, &mut find_data).is_err() {
                let _ = FindClose(handle);
                return;
            }
        }

        // Process up to iterations entries
        let mut last_size: u64 =
            ((find_data.nFileSizeHigh as u64) << 32) | (find_data.nFileSizeLow as u64);
        black_box(&find_data.cFileName);
        black_box(find_data.nFileSizeHigh);
        black_box(find_data.nFileSizeLow);

        for _ in 1..params.iterations {
            if FindNextFileW(handle, &mut find_data).is_err() {
                break;
            }
            last_size =
                ((find_data.nFileSizeHigh as u64) << 32) | (find_data.nFileSizeLow as u64);
            black_box(&find_data.cFileName);
            black_box(find_data.nFileSizeHigh);
            black_box(find_data.nFileSizeLow);
        }

        scratch.absorb(&last_size.to_ne_bytes());
        black_box(last_size);
        let _ = FindClose(handle);
    }
}

fn get_module_handles(
    params: &TaskParams,
    rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
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

    let start_idx = work.derive_usize(0) % 12;
    let mut last_handle: usize = 0;
    unsafe {
        for _ in 0..params.iterations {
            let idx = (start_idx + rng.gen_range(0..dll_names.len())) % dll_names.len();
            let result = GetModuleHandleW(dll_names[idx]);
            match result {
                Ok(h) => {
                    last_handle = h.0 as usize;
                    black_box(h);
                }
                Err(_) => {
                    black_box(0u32);
                }
            }
        }
        scratch.absorb(&last_handle.to_ne_bytes());
        black_box(last_handle);
    }
}

fn virtual_query_memory(
    params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let bias = work.derive_usize(0) % 65536;
    unsafe {
        let mut addr: usize = bias;
        let info_size = std::mem::size_of::<MEMORY_BASIC_INFORMATION>();
        let mut last_info = MEMORY_BASIC_INFORMATION::default();

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
            last_info = info;
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

        let info_bytes: &[u8] = std::slice::from_raw_parts(
            &last_info as *const MEMORY_BASIC_INFORMATION as *const u8,
            std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
        );
        scratch.absorb(info_bytes);
        black_box(&last_info);
    }
}

fn get_system_directories(
    _params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let loops = _params.call_depth + (work.blend_seed() % 2) as usize;
    unsafe {
        for _ in 0..loops {
            let mut sys_dir = [0u16; 260];
            let len = GetSystemDirectoryW(Some(&mut sys_dir));
            if len > 0 {
                let dir_bytes: &[u8] = std::slice::from_raw_parts(
                    sys_dir.as_ptr() as *const u8,
                    (len as usize) * 2,
                );
                scratch.absorb(dir_bytes);
                black_box(&sys_dir[..len as usize]);
            }

            let mut win_dir = [0u16; 260];
            let len = GetWindowsDirectoryW(Some(&mut win_dir));
            if len > 0 {
                let dir_bytes: &[u8] = std::slice::from_raw_parts(
                    win_dir.as_ptr() as *const u8,
                    (len as usize) * 2,
                );
                scratch.absorb(dir_bytes);
                black_box(&win_dir[..len as usize]);
            }
        }
    }
}

fn get_process_thread_ids(
    params: &TaskParams,
    _rng: &mut ThreadRng,
    _work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    unsafe {
        let mut last_pid: u32 = 0;
        let mut last_tid: u32 = 0;
        for _ in 0..params.iterations {
            let pid = GetCurrentProcessId();
            last_pid = pid;
            black_box(pid);

            let tid = GetCurrentThreadId();
            last_tid = tid;
            black_box(tid);
        }
        scratch.absorb(&last_pid.to_ne_bytes());
        scratch.absorb(&last_tid.to_ne_bytes());
        black_box(last_pid);
        black_box(last_tid);
    }
}
