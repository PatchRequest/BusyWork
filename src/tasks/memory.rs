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
        TaskDescriptor {
            name: "heap_fragmentation",
            category: Categories::MEMORY,
            func: heap_fragmentation,
        },
        TaskDescriptor {
            name: "ring_buffer_ops",
            category: Categories::MEMORY,
            func: ring_buffer_ops,
        },
        TaskDescriptor {
            name: "binary_search_repeated",
            category: Categories::MEMORY,
            func: binary_search_repeated,
        },
        TaskDescriptor {
            name: "reverse_buffer",
            category: Categories::MEMORY,
            func: reverse_buffer,
        },
        TaskDescriptor {
            name: "interleave_buffers",
            category: Categories::MEMORY,
            func: interleave_buffers,
        },
        TaskDescriptor {
            name: "scatter_gather",
            category: Categories::MEMORY,
            func: scatter_gather,
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

fn heap_fragmentation(params: &TaskParams, rng: &mut ThreadRng) {
    let count = params.iterations.min(500);
    let mut buffers: Vec<Option<Vec<u8>>> = Vec::with_capacity(count);

    // Allocate many small random-sized buffers
    for _ in 0..count {
        let size = rng.gen_range(16..=4096);
        let mut buf = vec![0u8; size];
        rng.fill_bytes(&mut buf);
        buffers.push(Some(buf));
    }

    // Drop roughly 50% at random
    for slot in buffers.iter_mut() {
        if rng.gen_bool(0.5) {
            *slot = None;
        }
    }

    // Allocate more to fill gaps
    let mut total_bytes = 0usize;
    for slot in buffers.iter_mut() {
        if slot.is_none() {
            let size = rng.gen_range(16..=4096);
            let mut buf = vec![0u8; size];
            rng.fill_bytes(&mut buf);
            total_bytes += size;
            *slot = Some(buf);
        } else {
            total_bytes += slot.as_ref().unwrap().len();
        }
    }

    black_box(total_bytes);
    black_box(&buffers);
}

fn ring_buffer_ops(params: &TaskParams, rng: &mut ThreadRng) {
    let size = params.buffer_size.min(1_048_576).max(1);
    let mut buffer = vec![0u8; size];
    let mut write_pos: usize = 0;
    let mut read_pos: usize = 0;

    for _ in 0..params.iterations {
        // Write a random byte at write_pos
        buffer[write_pos] = rng.gen();
        write_pos = (write_pos + 1) % size;

        // Read at read_pos
        let val = buffer[read_pos];
        black_box(val);
        read_pos = (read_pos + 1) % size;
    }

    black_box(&buffer);
}

fn binary_search_repeated(params: &TaskParams, rng: &mut ThreadRng) {
    let elem_count = (params.buffer_size / 8).max(1);
    let mut data: Vec<u64> = (0..elem_count).map(|_| rng.gen()).collect();
    data.sort_unstable();

    let mut hits = 0usize;
    for _ in 0..params.iterations {
        let target: u64 = rng.gen();
        if data.binary_search(&target).is_ok() {
            hits += 1;
        }
    }

    black_box(hits);
}

fn reverse_buffer(params: &TaskParams, rng: &mut ThreadRng) {
    let size = params.buffer_size.min(1_048_576).max(1);
    let mut buf = vec![0u8; size];
    rng.fill_bytes(&mut buf);

    for _ in 0..params.iterations {
        buf.reverse();
    }

    black_box(&buf);
}

fn interleave_buffers(params: &TaskParams, rng: &mut ThreadRng) {
    let half_size = (params.buffer_size / 2).max(1);
    for _ in 0..params.call_depth {
        let mut buf_a = vec![0u8; half_size];
        let mut buf_b = vec![0u8; half_size];
        rng.fill_bytes(&mut buf_a);
        rng.fill_bytes(&mut buf_b);

        let mut interleaved = Vec::with_capacity(half_size * 2);
        for i in 0..half_size {
            interleaved.push(buf_a[i]);
            interleaved.push(buf_b[i]);
        }

        black_box(&interleaved);
    }
}

fn scatter_gather(params: &TaskParams, rng: &mut ThreadRng) {
    let size = params.buffer_size.min(1_048_576).max(1);
    let index_count = (size / 4).max(1);

    let mut buffer = vec![0u8; size];
    rng.fill_bytes(&mut buffer);

    for _ in 0..params.iterations.min(100) {
        // Generate random index list
        let indices: Vec<usize> = (0..index_count).map(|_| rng.gen_range(0..size)).collect();

        // Gather: read at random indices into contiguous output
        let gathered: Vec<u8> = indices.iter().map(|&idx| buffer[idx]).collect();
        black_box(&gathered);

        // Scatter: write from contiguous input to random indices
        let input: Vec<u8> = (0..index_count).map(|_| rng.gen()).collect();
        for (i, &idx) in indices.iter().enumerate() {
            buffer[idx] = input[i];
        }
    }

    black_box(&buffer);
}
