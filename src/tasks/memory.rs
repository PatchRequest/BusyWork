use crate::categories::Categories;
use crate::tasks::{TaskDescriptor, TaskParams};
use crate::workdata::WorkData;
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

fn alloc_touch_free(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let size = params.buffer_size.min(1_048_576);
    for _ in 0..params.iterations.min(100) {
        let mut buf = vec![0u8; size];
        for offset in (0..size).step_by(4096) {
            buf[offset] = rng.gen();
        }
        work.blend_into(&mut buf);
        black_box(&buf);
    }
}

fn memcpy_chain(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let size = params.buffer_size.min(1_048_576);
    let mut src = vec![0u8; size];
    let mut dst = vec![0u8; size];
    rng.fill_bytes(&mut src);
    work.blend_into(&mut src);
    for _ in 0..params.iterations.min(1000) {
        dst.copy_from_slice(&src);
        std::mem::swap(&mut src, &mut dst);
    }
    black_box(&src);
}

fn sort_random_memory(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let size = (params.buffer_size / 8).max(1);
    let seed = work.blend_seed();
    for _ in 0..params.call_depth {
        let mut data: Vec<u64> = (0..size).map(|_| rng.gen::<u64>() ^ seed).collect();
        data.sort_unstable();
        black_box(&data);
    }
}

fn pattern_fill_verify(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let size = params.buffer_size.min(1_048_576);
    let mut buf = vec![0u8; size];
    let wb = work.as_bytes();
    for round in 0..params.iterations.min(100) {
        let base_pattern: u8 = rng.gen();
        let pattern = if !wb.is_empty() {
            base_pattern ^ wb[round % wb.len()]
        } else {
            base_pattern
        };
        buf.fill(pattern);
        let valid = buf.iter().all(|&b| b == pattern);
        black_box(valid);
    }
}

fn heap_fragmentation(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let count = params.iterations.min(500);
    let mut buffers: Vec<Option<Vec<u8>>> = Vec::with_capacity(count);

    for _ in 0..count {
        let size = rng.gen_range(16..=4096);
        let mut buf = vec![0u8; size];
        rng.fill_bytes(&mut buf);
        work.blend_into(&mut buf);
        buffers.push(Some(buf));
    }

    for slot in buffers.iter_mut() {
        if rng.gen_bool(0.5) {
            *slot = None;
        }
    }

    let mut total_bytes = 0usize;
    for slot in buffers.iter_mut() {
        if slot.is_none() {
            let size = rng.gen_range(16..=4096);
            let mut buf = vec![0u8; size];
            rng.fill_bytes(&mut buf);
            work.blend_into(&mut buf);
            total_bytes += size;
            *slot = Some(buf);
        } else {
            total_bytes += slot.as_ref().unwrap().len();
        }
    }

    black_box(total_bytes);
    black_box(&buffers);
}

fn ring_buffer_ops(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let size = params.buffer_size.min(1_048_576).max(1);
    let mut buffer = vec![0u8; size];
    work.blend_into(&mut buffer);
    let mut write_pos: usize = 0;
    let mut read_pos: usize = 0;

    for _ in 0..params.iterations {
        buffer[write_pos] = rng.gen();
        write_pos = (write_pos + 1) % size;

        let val = buffer[read_pos];
        black_box(val);
        read_pos = (read_pos + 1) % size;
    }

    black_box(&buffer);
}

fn binary_search_repeated(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let elem_count = (params.buffer_size / 8).max(1);
    let seed = work.blend_seed();
    let mut data: Vec<u64> = (0..elem_count).map(|_| rng.gen::<u64>() ^ seed).collect();
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

fn reverse_buffer(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let size = params.buffer_size.min(1_048_576).max(1);
    let mut buf = vec![0u8; size];
    rng.fill_bytes(&mut buf);
    work.blend_into(&mut buf);

    for _ in 0..params.iterations {
        buf.reverse();
    }

    black_box(&buf);
}

fn interleave_buffers(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let half_size = (params.buffer_size / 2).max(1);
    for _ in 0..params.call_depth {
        let mut buf_a = vec![0u8; half_size];
        let mut buf_b = vec![0u8; half_size];
        rng.fill_bytes(&mut buf_a);
        rng.fill_bytes(&mut buf_b);
        work.blend_into(&mut buf_a);

        let mut interleaved = Vec::with_capacity(half_size * 2);
        for i in 0..half_size {
            interleaved.push(buf_a[i]);
            interleaved.push(buf_b[i]);
        }

        black_box(&interleaved);
    }
}

fn scatter_gather(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let size = params.buffer_size.min(1_048_576).max(1);
    let index_count = (size / 4).max(1);

    let mut buffer = vec![0u8; size];
    rng.fill_bytes(&mut buffer);
    work.blend_into(&mut buffer);

    for _ in 0..params.iterations.min(100) {
        let indices: Vec<usize> = (0..index_count).map(|_| rng.gen_range(0..size)).collect();

        let gathered: Vec<u8> = indices.iter().map(|&idx| buffer[idx]).collect();
        black_box(&gathered);

        let input: Vec<u8> = (0..index_count).map(|_| rng.gen()).collect();
        for (i, &idx) in indices.iter().enumerate() {
            buffer[idx] = input[i];
        }
    }

    black_box(&buffer);
}

#[cfg(test)]
mod tests {
    use crate::workdata::WorkData;

    #[test]
    fn alloc_buffer_incorporates_work() {
        let mut buf = vec![0u8; 4096];
        let mut work = WorkData::new();
        work.feed(&0xDEADBEEFu32);
        work.blend_into(&mut buf);
        assert!(buf.iter().any(|&b| b != 0));
    }

    #[test]
    fn memcpy_source_modified() {
        let mut src = vec![0u8; 1024];
        let mut work = WorkData::new();
        work.feed("memcpy_context");
        work.blend_into(&mut src);
        assert!(src.iter().any(|&b| b != 0));
    }

    #[test]
    fn sort_seed_nonzero() {
        let mut work = WorkData::new();
        work.feed(&42u32);
        assert_ne!(work.blend_seed(), 0);
    }

    #[test]
    fn pattern_xor_correct() {
        let mut work = WorkData::new();
        work.feed(&0x55u8);
        let wb = work.as_bytes();
        assert_eq!(0xAAu8 ^ wb[0], 0xFF);
    }

    #[test]
    fn ring_buffer_initial_blend() {
        let mut buffer = vec![0u8; 256];
        let mut work = WorkData::new();
        work.feed(&[1, 2, 3, 4]);
        work.blend_into(&mut buffer);
        assert_eq!(&buffer[..4], &[1, 2, 3, 4]);
    }

    #[test]
    fn binary_search_seed_nonzero() {
        let mut work = WorkData::new();
        work.feed(&42u32);
        assert_ne!(work.blend_seed(), 0);
    }

    #[test]
    fn interleave_blends_only_buf_a() {
        let mut a = vec![0u8; 64];
        let b = vec![0u8; 64];
        let mut work = WorkData::new();
        work.feed(&[0xAA; 8]);
        work.blend_into(&mut a);
        assert!(a.iter().any(|&v| v != 0));
        assert!(b.iter().all(|&v| v == 0));
    }

    #[test]
    fn single_byte_cycles_across_buffer() {
        let mut work = WorkData::new();
        work.feed(&[0xAB]);
        let mut buf = vec![0u8; 8];
        work.blend_into(&mut buf);
        assert!(buf.iter().all(|&b| b == 0xAB));
    }

    #[test]
    fn blend_into_reversible() {
        let mut work = WorkData::new();
        work.feed(&0xCAFEBABEu32);
        let original = vec![42u8; 256];
        let mut buf = original.clone();
        work.blend_into(&mut buf);
        assert_ne!(buf, original);
        work.blend_into(&mut buf);
        assert_eq!(buf, original);
    }
}
