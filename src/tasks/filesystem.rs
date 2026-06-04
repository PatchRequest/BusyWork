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
