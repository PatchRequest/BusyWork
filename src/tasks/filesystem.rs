use crate::categories::Categories;
use crate::tasks::{TaskDescriptor, TaskParams};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::hint::black_box;

pub fn register() -> Vec<TaskDescriptor> {
    vec![
        TaskDescriptor {
            name: "enumerate_system_dir",
            category: Categories::FILESYSTEM,
            func: enumerate_system_dir,
        },
        TaskDescriptor {
            name: "enumerate_temp_dir",
            category: Categories::FILESYSTEM,
            func: enumerate_temp_dir,
        },
        TaskDescriptor {
            name: "stat_system_files",
            category: Categories::FILESYSTEM,
            func: stat_system_files,
        },
        TaskDescriptor {
            name: "read_small_files",
            category: Categories::FILESYSTEM,
            func: read_small_files,
        },
        TaskDescriptor {
            name: "enumerate_program_files",
            category: Categories::FILESYSTEM,
            func: enumerate_program_files,
        },
        TaskDescriptor {
            name: "enumerate_fonts",
            category: Categories::FILESYSTEM,
            func: enumerate_fonts,
        },
        TaskDescriptor {
            name: "enumerate_drivers",
            category: Categories::FILESYSTEM,
            func: enumerate_drivers,
        },
        TaskDescriptor {
            name: "enumerate_prefetch",
            category: Categories::FILESYSTEM,
            func: enumerate_prefetch,
        },
        TaskDescriptor {
            name: "enumerate_logs",
            category: Categories::FILESYSTEM,
            func: enumerate_logs,
        },
        TaskDescriptor {
            name: "stat_common_dlls",
            category: Categories::FILESYSTEM,
            func: stat_common_dlls,
        },
        TaskDescriptor {
            name: "read_system_files_varied",
            category: Categories::FILESYSTEM,
            func: read_system_files_varied,
        },
        TaskDescriptor {
            name: "enumerate_user_profile",
            category: Categories::FILESYSTEM,
            func: enumerate_user_profile,
        },
    ]
}

fn enumerate_system_dir(params: &TaskParams, _rng: &mut ThreadRng) {
    let _ = (|| -> std::io::Result<()> {
        let dir = std::fs::read_dir(r"C:\Windows\System32")?;
        for (i, entry) in dir.enumerate() {
            if i >= params.iterations {
                break;
            }
            let entry = entry?;
            let meta = entry.metadata()?;
            black_box(meta.len());
        }
        Ok(())
    })();
}

fn enumerate_temp_dir(params: &TaskParams, _rng: &mut ThreadRng) {
    let _ = (|| -> std::io::Result<()> {
        let dir = std::fs::read_dir(std::env::temp_dir())?;
        for (i, entry) in dir.enumerate() {
            if i >= params.iterations {
                break;
            }
            let entry = entry?;
            let meta = entry.metadata()?;
            black_box(meta.len());
        }
        Ok(())
    })();
}

fn stat_system_files(params: &TaskParams, rng: &mut ThreadRng) {
    let paths = [
        r"C:\Windows\explorer.exe",
        r"C:\Windows\notepad.exe",
        r"C:\Windows\System32\kernel32.dll",
        r"C:\Windows\System32\ntdll.dll",
        r"C:\Windows\System32\user32.dll",
        r"C:\Windows\System32\advapi32.dll",
        r"C:\Windows\System32\ws2_32.dll",
        r"C:\Windows\System32\cmd.exe",
    ];
    for _ in 0..params.iterations.min(200) {
        if let Some(path) = paths.choose(rng) {
            if let Ok(meta) = std::fs::metadata(path) {
                black_box(meta.len());
            }
        }
    }
}

fn read_small_files(params: &TaskParams, rng: &mut ThreadRng) {
    let paths = [
        r"C:\Windows\System32\drivers\etc\hosts",
        r"C:\Windows\System32\drivers\etc\services",
        r"C:\Windows\System32\drivers\etc\protocol",
        r"C:\Windows\System32\drivers\etc\networks",
    ];
    for _ in 0..params.call_depth {
        if let Some(path) = paths.choose(rng) {
            if let Ok(data) = std::fs::read(path) {
                black_box(data.len());
            }
        }
    }
}

fn enumerate_program_files(params: &TaskParams, _rng: &mut ThreadRng) {
    let _ = (|| -> std::io::Result<()> {
        let dirs = [r"C:\Program Files", r"C:\Program Files (x86)"];
        let mut count = 0usize;
        for dir_path in &dirs {
            if let Ok(dir) = std::fs::read_dir(dir_path) {
                for entry in dir {
                    if count >= params.iterations {
                        return Ok(());
                    }
                    let entry = entry?;
                    if let Ok(meta) = entry.metadata() {
                        black_box(meta.len());
                        black_box(meta.is_dir());
                    }
                    count += 1;
                }
            }
        }
        Ok(())
    })();
}

fn enumerate_fonts(params: &TaskParams, _rng: &mut ThreadRng) {
    let _ = (|| -> std::io::Result<()> {
        let dir = std::fs::read_dir(r"C:\Windows\Fonts")?;
        for (i, entry) in dir.enumerate() {
            if i >= params.iterations {
                break;
            }
            let entry = entry?;
            black_box(entry.file_name());
            if let Ok(meta) = entry.metadata() {
                black_box(meta.len());
            }
        }
        Ok(())
    })();
}

fn enumerate_drivers(params: &TaskParams, _rng: &mut ThreadRng) {
    let _ = (|| -> std::io::Result<()> {
        let dir = std::fs::read_dir(r"C:\Windows\System32\drivers")?;
        for (i, entry) in dir.enumerate() {
            if i >= params.iterations {
                break;
            }
            let entry = entry?;
            if let Ok(meta) = entry.metadata() {
                black_box(meta.len());
            }
        }
        Ok(())
    })();
}

fn enumerate_prefetch(params: &TaskParams, _rng: &mut ThreadRng) {
    let _ = (|| -> std::io::Result<()> {
        let dir = std::fs::read_dir(r"C:\Windows\Prefetch")?;
        for (i, entry) in dir.enumerate() {
            if i >= params.iterations {
                break;
            }
            let entry = entry?;
            black_box(entry.file_name());
            if let Ok(meta) = entry.metadata() {
                black_box(meta.len());
            }
        }
        Ok(())
    })();
}

fn enumerate_logs(params: &TaskParams, _rng: &mut ThreadRng) {
    let _ = (|| -> std::io::Result<()> {
        let mut count = 0usize;
        let base = std::path::Path::new(r"C:\Windows\Logs");
        let dir = std::fs::read_dir(base)?;
        for entry in dir {
            if count >= params.iterations {
                break;
            }
            let entry = entry?;
            black_box(entry.file_name());
            if let Ok(meta) = entry.metadata() {
                black_box(meta.len());
                if meta.is_dir() {
                    if let Ok(subdir) = std::fs::read_dir(entry.path()) {
                        for sub_entry in subdir {
                            if count >= params.iterations {
                                break;
                            }
                            if let Ok(sub_entry) = sub_entry {
                                black_box(sub_entry.file_name());
                                if let Ok(sub_meta) = sub_entry.metadata() {
                                    black_box(sub_meta.len());
                                }
                            }
                            count += 1;
                        }
                    }
                }
            }
            count += 1;
        }
        Ok(())
    })();
}

fn stat_common_dlls(params: &TaskParams, rng: &mut ThreadRng) {
    let dlls = [
        r"C:\Windows\System32\kernel32.dll",
        r"C:\Windows\System32\ntdll.dll",
        r"C:\Windows\System32\user32.dll",
        r"C:\Windows\System32\gdi32.dll",
        r"C:\Windows\System32\advapi32.dll",
        r"C:\Windows\System32\shell32.dll",
        r"C:\Windows\System32\ole32.dll",
        r"C:\Windows\System32\oleaut32.dll",
        r"C:\Windows\System32\msvcrt.dll",
        r"C:\Windows\System32\ws2_32.dll",
        r"C:\Windows\System32\crypt32.dll",
        r"C:\Windows\System32\secur32.dll",
        r"C:\Windows\System32\winhttp.dll",
        r"C:\Windows\System32\wininet.dll",
        r"C:\Windows\System32\mswsock.dll",
        r"C:\Windows\System32\rpcrt4.dll",
        r"C:\Windows\System32\combase.dll",
        r"C:\Windows\System32\bcrypt.dll",
        r"C:\Windows\System32\ncrypt.dll",
        r"C:\Windows\System32\shlwapi.dll",
    ];
    for _ in 0..params.iterations {
        if let Some(path) = dlls.choose(rng) {
            if let Ok(meta) = std::fs::metadata(path) {
                black_box(meta.len());
            }
        }
    }
}

fn read_system_files_varied(params: &TaskParams, rng: &mut ThreadRng) {
    let paths = [
        r"C:\Windows\win.ini",
        r"C:\Windows\system.ini",
        r"C:\Windows\System32\drivers\etc\hosts",
        r"C:\Windows\System32\drivers\etc\services",
        r"C:\Windows\System32\drivers\etc\protocol",
    ];
    for _ in 0..params.iterations {
        if let Some(path) = paths.choose(rng) {
            if let Ok(data) = std::fs::read(path) {
                black_box(data.len());
            }
        }
    }
}

fn enumerate_user_profile(params: &TaskParams, _rng: &mut ThreadRng) {
    let _ = (|| -> std::io::Result<()> {
        let profile = std::env::var("USERPROFILE").map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "USERPROFILE not set")
        })?;
        let dir = std::fs::read_dir(profile)?;
        for (i, entry) in dir.enumerate() {
            if i >= params.iterations {
                break;
            }
            let entry = entry?;
            black_box(entry.file_name());
            if let Ok(meta) = entry.metadata() {
                black_box(meta.len());
                black_box(meta.is_dir());
            }
        }
        Ok(())
    })();
}
