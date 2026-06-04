use crate::categories::Categories;
use crate::tasks::{TaskDescriptor, TaskParams};
use rand::rngs::ThreadRng;
use rand::{Rng, RngCore};
use std::hint::black_box;

pub fn register() -> Vec<TaskDescriptor> {
    vec![
        TaskDescriptor {
            name: "alloc_touch_free",
            category: Categories::MEMORY,
            func: alloc_touch_free,
        },
        TaskDescriptor {
            name: "memcpy_chain",
            category: Categories::MEMORY,
            func: memcpy_chain,
        },
        TaskDescriptor {
            name: "sort_random_memory",
            category: Categories::MEMORY,
            func: sort_random_memory,
        },
        TaskDescriptor {
            name: "pattern_fill_verify",
            category: Categories::MEMORY,
            func: pattern_fill_verify,
        },
    ]
}

fn alloc_touch_free(params: &TaskParams, rng: &mut ThreadRng) {
    let size = params.buffer_size.min(1_048_576);
    for _ in 0..params.iterations.min(100) {
        let mut buf = vec![0u8; size];
        for offset in (0..size).step_by(4096) {
            buf[offset] = rng.gen();
        }
        black_box(&buf);
    }
}

fn memcpy_chain(params: &TaskParams, rng: &mut ThreadRng) {
    let size = params.buffer_size.min(1_048_576);
    let mut src = vec![0u8; size];
    let mut dst = vec![0u8; size];
    rng.fill_bytes(&mut src);
    for _ in 0..params.iterations.min(1000) {
        dst.copy_from_slice(&src);
        std::mem::swap(&mut src, &mut dst);
    }
    black_box(&src);
}

fn sort_random_memory(params: &TaskParams, rng: &mut ThreadRng) {
    let size = (params.buffer_size / 8).max(1);
    for _ in 0..params.call_depth {
        let mut data: Vec<u64> = (0..size).map(|_| rng.gen()).collect();
        data.sort_unstable();
        black_box(&data);
    }
}

fn pattern_fill_verify(params: &TaskParams, rng: &mut ThreadRng) {
    let size = params.buffer_size.min(1_048_576);
    let mut buf = vec![0u8; size];
    for _ in 0..params.iterations.min(100) {
        let pattern: u8 = rng.gen();
        buf.fill(pattern);
        let valid = buf.iter().all(|&b| b == pattern);
        black_box(valid);
    }
}
