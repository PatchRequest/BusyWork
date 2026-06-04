use crate::categories::Categories;
use crate::tasks::{TaskDescriptor, TaskParams};
use rand::rngs::ThreadRng;
use std::hint::black_box;
use windows::Win32::System::Registry::*;

pub fn register() -> Vec<TaskDescriptor> {
    vec![
        TaskDescriptor {
            name: "read_installed_software",
            category: Categories::REGISTRY,
            func: read_installed_software,
        },
        TaskDescriptor {
            name: "read_system_info_reg",
            category: Categories::REGISTRY,
            func: read_system_info_reg,
        },
        TaskDescriptor {
            name: "enumerate_services_reg",
            category: Categories::REGISTRY,
            func: enumerate_services_reg,
        },
        TaskDescriptor {
            name: "read_timezone_info",
            category: Categories::REGISTRY,
            func: read_timezone_info,
        },
        TaskDescriptor {
            name: "read_environment_vars",
            category: Categories::REGISTRY,
            func: read_environment_vars,
        },
        TaskDescriptor {
            name: "read_network_config",
            category: Categories::REGISTRY,
            func: read_network_config,
        },
        TaskDescriptor {
            name: "read_hardware_info",
            category: Categories::REGISTRY,
            func: read_hardware_info,
        },
        TaskDescriptor {
            name: "read_font_list",
            category: Categories::REGISTRY,
            func: read_font_list,
        },
        TaskDescriptor {
            name: "read_startup_programs",
            category: Categories::REGISTRY,
            func: read_startup_programs,
        },
        TaskDescriptor {
            name: "read_file_associations",
            category: Categories::REGISTRY,
            func: read_file_associations,
        },
    ]
}

unsafe fn enum_subkeys(root: HKEY, subkey_path: &[u16], max_keys: usize) {
    let mut hkey = HKEY::default();
    let result = RegOpenKeyExW(
        root,
        windows::core::PCWSTR(subkey_path.as_ptr()),
        0,
        KEY_READ,
        &mut hkey,
    );
    if result.0 != 0 {
        return;
    }

    let mut name_buf = [0u16; 256];
    for i in 0..max_keys as u32 {
        let mut name_len = name_buf.len() as u32;
        let result = RegEnumKeyExW(
            hkey,
            i,
            windows::core::PWSTR(name_buf.as_mut_ptr()),
            &mut name_len,
            None,
            windows::core::PWSTR::null(),
            None,
            None,
        );
        if result.0 != 0 {
            break;
        }
        black_box(&name_buf[..name_len as usize]);
    }

    let _ = RegCloseKey(hkey);
}

unsafe fn read_string_value(hkey: HKEY, value_name: &[u16]) {
    let mut data_type = REG_VALUE_TYPE::default();
    let mut buf = [0u8; 512];
    let mut buf_len = buf.len() as u32;
    let result = RegQueryValueExW(
        hkey,
        windows::core::PCWSTR(value_name.as_ptr()),
        None,
        Some(&mut data_type),
        Some(buf.as_mut_ptr()),
        Some(&mut buf_len),
    );
    if result.0 == 0 {
        black_box(&buf[..buf_len as usize]);
    }
}

unsafe fn enum_values(hkey: HKEY, max_values: usize) {
    let mut name_buf = [0u16; 256];
    let mut data_buf = [0u8; 512];
    for i in 0..max_values as u32 {
        let mut name_len = name_buf.len() as u32;
        let mut data_len = data_buf.len() as u32;
        let mut data_type = 0u32;
        let result = RegEnumValueW(
            hkey,
            i,
            windows::core::PWSTR(name_buf.as_mut_ptr()),
            &mut name_len,
            None,
            Some(&mut data_type),
            Some(data_buf.as_mut_ptr()),
            Some(&mut data_len),
        );
        if result.0 != 0 {
            break;
        }
        black_box(&name_buf[..name_len as usize]);
        black_box(&data_buf[..data_len as usize]);
    }
}

fn read_installed_software(params: &TaskParams, _rng: &mut ThreadRng) {
    const PATH: &[u16] = &{
        let mut arr = [0u16; 60];
        let bytes = b"SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\0";
        let mut i = 0;
        while i < bytes.len() {
            arr[i] = bytes[i] as u16;
            i += 1;
        }
        arr
    };
    unsafe {
        enum_subkeys(HKEY_LOCAL_MACHINE, PATH, params.iterations);
    }
}

fn read_system_info_reg(params: &TaskParams, _rng: &mut ThreadRng) {
    const PATH: &[u16] = &{
        let mut arr = [0u16; 50];
        let bytes = b"SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\0";
        let mut i = 0;
        while i < bytes.len() {
            arr[i] = bytes[i] as u16;
            i += 1;
        }
        arr
    };
    unsafe {
        let mut hkey = HKEY::default();
        let result = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            windows::core::PCWSTR(PATH.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        if result.0 != 0 {
            return;
        }

        let value_names: &[&[u16]] = &[
            &encode_wide_const::<20>(b"ProductName\0"),
            &encode_wide_const::<20>(b"CurrentBuild\0"),
            &encode_wide_const::<20>(b"EditionID\0"),
        ];
        for _ in 0..params.call_depth {
            for name in value_names {
                read_string_value(hkey, name);
            }
        }

        let _ = RegCloseKey(hkey);
    }
}

fn enumerate_services_reg(params: &TaskParams, _rng: &mut ThreadRng) {
    const PATH: &[u16] = &{
        let mut arr = [0u16; 50];
        let bytes = b"SYSTEM\\CurrentControlSet\\Services\0";
        let mut i = 0;
        while i < bytes.len() {
            arr[i] = bytes[i] as u16;
            i += 1;
        }
        arr
    };
    unsafe {
        enum_subkeys(HKEY_LOCAL_MACHINE, PATH, params.iterations);
    }
}

fn read_timezone_info(params: &TaskParams, _rng: &mut ThreadRng) {
    const PATH: &[u16] = &{
        let mut arr = [0u16; 60];
        let bytes = b"SYSTEM\\CurrentControlSet\\Control\\TimeZoneInformation\0";
        let mut i = 0;
        while i < bytes.len() {
            arr[i] = bytes[i] as u16;
            i += 1;
        }
        arr
    };
    unsafe {
        let mut hkey = HKEY::default();
        let result = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            windows::core::PCWSTR(PATH.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        if result.0 != 0 {
            return;
        }

        let value_names: &[&[u16]] = &[
            &encode_wide_const::<30>(b"TimeZoneKeyName\0"),
            &encode_wide_const::<20>(b"StandardName\0"),
        ];
        for _ in 0..params.call_depth {
            for name in value_names {
                read_string_value(hkey, name);
            }
        }

        let _ = RegCloseKey(hkey);
    }
}

fn read_environment_vars(params: &TaskParams, _rng: &mut ThreadRng) {
    const PATH: &[u16] = &{
        let mut arr = [0u16; 70];
        let bytes = b"SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment\0";
        let mut i = 0;
        while i < bytes.len() {
            arr[i] = bytes[i] as u16;
            i += 1;
        }
        arr
    };
    unsafe {
        let mut hkey = HKEY::default();
        let result = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            windows::core::PCWSTR(PATH.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        if result.0 != 0 {
            return;
        }

        let value_names: &[&[u16]] = &[
            &encode_wide_const::<10>(b"Path\0"),
            &encode_wide_const::<10>(b"TEMP\0"),
            &encode_wide_const::<10>(b"TMP\0"),
            &encode_wide_const::<10>(b"OS\0"),
            &encode_wide_const::<30>(b"PROCESSOR_ARCHITECTURE\0"),
            &encode_wide_const::<20>(b"NUMBER_OF_PROCESSORS\0"),
            &encode_wide_const::<20>(b"PATHEXT\0"),
            &encode_wide_const::<15>(b"ComSpec\0"),
            &encode_wide_const::<15>(b"windir\0"),
        ];
        for _ in 0..params.call_depth {
            for name in value_names {
                read_string_value(hkey, name);
            }
        }

        let _ = RegCloseKey(hkey);
    }
}

fn read_network_config(params: &TaskParams, _rng: &mut ThreadRng) {
    const PATH: &[u16] = &{
        let mut arr = [0u16; 60];
        let bytes = b"SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\0";
        let mut i = 0;
        while i < bytes.len() {
            arr[i] = bytes[i] as u16;
            i += 1;
        }
        arr
    };
    const INTERFACES_PATH: &[u16] = &{
        let mut arr = [0u16; 75];
        let bytes = b"SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters\\Interfaces\0";
        let mut i = 0;
        while i < bytes.len() {
            arr[i] = bytes[i] as u16;
            i += 1;
        }
        arr
    };
    unsafe {
        let mut hkey = HKEY::default();
        let result = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            windows::core::PCWSTR(PATH.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        if result.0 == 0 {
            let value_names: &[&[u16]] = &[
                &encode_wide_const::<15>(b"Hostname\0"),
                &encode_wide_const::<15>(b"Domain\0"),
                &encode_wide_const::<15>(b"SearchList\0"),
                &encode_wide_const::<20>(b"NV Hostname\0"),
                &encode_wide_const::<20>(b"ICSDomain\0"),
            ];
            for _ in 0..params.call_depth {
                for name in value_names {
                    read_string_value(hkey, name);
                }
            }
            let _ = RegCloseKey(hkey);
        }

        enum_subkeys(HKEY_LOCAL_MACHINE, INTERFACES_PATH, params.iterations);
    }
}

fn read_hardware_info(params: &TaskParams, _rng: &mut ThreadRng) {
    const PATH: &[u16] = &{
        let mut arr = [0u16; 60];
        let bytes = b"HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0\0";
        let mut i = 0;
        while i < bytes.len() {
            arr[i] = bytes[i] as u16;
            i += 1;
        }
        arr
    };
    unsafe {
        let mut hkey = HKEY::default();
        let result = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            windows::core::PCWSTR(PATH.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        if result.0 != 0 {
            return;
        }

        let value_names: &[&[u16]] = &[
            &encode_wide_const::<25>(b"ProcessorNameString\0"),
            &encode_wide_const::<10>(b"~MHz\0"),
            &encode_wide_const::<15>(b"Identifier\0"),
            &encode_wide_const::<20>(b"VendorIdentifier\0"),
            &encode_wide_const::<20>(b"FeatureSet\0"),
            &encode_wide_const::<15>(b"Update Status\0"),
        ];
        for _ in 0..params.call_depth {
            for name in value_names {
                read_string_value(hkey, name);
            }
        }

        let _ = RegCloseKey(hkey);
    }
}

fn read_font_list(params: &TaskParams, _rng: &mut ThreadRng) {
    const PATH: &[u16] = &{
        let mut arr = [0u16; 60];
        let bytes = b"SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Fonts\0";
        let mut i = 0;
        while i < bytes.len() {
            arr[i] = bytes[i] as u16;
            i += 1;
        }
        arr
    };
    unsafe {
        let mut hkey = HKEY::default();
        let result = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            windows::core::PCWSTR(PATH.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        if result.0 != 0 {
            return;
        }

        enum_values(hkey, params.iterations);

        let _ = RegCloseKey(hkey);
    }
}

fn read_startup_programs(params: &TaskParams, _rng: &mut ThreadRng) {
    const PATH_HKLM: &[u16] = &{
        let mut arr = [0u16; 50];
        let bytes = b"SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run\0";
        let mut i = 0;
        while i < bytes.len() {
            arr[i] = bytes[i] as u16;
            i += 1;
        }
        arr
    };
    const PATH_HKCU: &[u16] = &{
        let mut arr = [0u16; 50];
        let bytes = b"SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run\0";
        let mut i = 0;
        while i < bytes.len() {
            arr[i] = bytes[i] as u16;
            i += 1;
        }
        arr
    };
    unsafe {
        // Read HKLM\...\Run
        let mut hkey = HKEY::default();
        let result = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            windows::core::PCWSTR(PATH_HKLM.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        if result.0 == 0 {
            enum_values(hkey, params.iterations);
            let _ = RegCloseKey(hkey);
        }

        // Read HKCU\...\Run
        let mut hkey = HKEY::default();
        let result = RegOpenKeyExW(
            HKEY_CURRENT_USER,
            windows::core::PCWSTR(PATH_HKCU.as_ptr()),
            0,
            KEY_READ,
            &mut hkey,
        );
        if result.0 == 0 {
            enum_values(hkey, params.iterations);
            let _ = RegCloseKey(hkey);
        }
    }
}

fn read_file_associations(params: &TaskParams, _rng: &mut ThreadRng) {
    unsafe {
        let mut name_buf = [0u16; 256];
        for i in 0..params.iterations as u32 {
            let mut name_len = name_buf.len() as u32;
            let result = RegEnumKeyExW(
                HKEY_CLASSES_ROOT,
                i,
                windows::core::PWSTR(name_buf.as_mut_ptr()),
                &mut name_len,
                None,
                windows::core::PWSTR::null(),
                None,
                None,
            );
            if result.0 != 0 {
                break;
            }
            // Only process entries starting with '.'
            if name_len > 0 && name_buf[0] == b'.' as u16 {
                black_box(&name_buf[..name_len as usize]);
            }
        }
    }
}

const fn encode_wide_const<const N: usize>(bytes: &[u8]) -> [u16; N] {
    let mut arr = [0u16; N];
    let mut i = 0;
    while i < bytes.len() && i < N {
        arr[i] = bytes[i] as u16;
        i += 1;
    }
    arr
}
