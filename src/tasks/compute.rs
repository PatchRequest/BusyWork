use crate::categories::Categories;
use crate::tasks::{ScratchBuffer, TaskDescriptor, TaskParams};
use crate::workdata::WorkData;
use flate2::write::{DeflateDecoder, DeflateEncoder};
use flate2::Compression;
use md5::Md5;
use rand::rngs::ThreadRng;
use rand::{Rng, RngCore};
use sha2::{Digest, Sha256};
use std::hint::black_box;
use std::io::Write;

pub fn register() -> Vec<TaskDescriptor> {
    vec![
        TaskDescriptor {
            name: "hash_sha256_loop",
            category: Categories::COMPUTE,
            func: hash_sha256_loop,
        },
        TaskDescriptor {
            name: "hash_md5_loop",
            category: Categories::COMPUTE,
            func: hash_md5_loop,
        },
        TaskDescriptor {
            name: "prime_sieve",
            category: Categories::COMPUTE,
            func: prime_sieve,
        },
        TaskDescriptor {
            name: "matrix_multiply",
            category: Categories::COMPUTE,
            func: matrix_multiply,
        },
        TaskDescriptor {
            name: "sort_random_arrays",
            category: Categories::COMPUTE,
            func: sort_random_arrays,
        },
        TaskDescriptor {
            name: "compress_decompress",
            category: Categories::COMPUTE,
            func: compress_decompress,
        },
        TaskDescriptor {
            name: "fibonacci_sequence",
            category: Categories::COMPUTE,
            func: fibonacci_sequence,
        },
        TaskDescriptor {
            name: "xor_cipher",
            category: Categories::COMPUTE,
            func: xor_cipher,
        },
        TaskDescriptor {
            name: "collatz_sequence",
            category: Categories::COMPUTE,
            func: collatz_sequence,
        },
        TaskDescriptor {
            name: "string_operations",
            category: Categories::COMPUTE,
            func: string_operations,
        },
        TaskDescriptor {
            name: "bubble_sort",
            category: Categories::COMPUTE,
            func: bubble_sort,
        },
        TaskDescriptor {
            name: "bitwise_operations",
            category: Categories::COMPUTE,
            func: bitwise_operations,
        },
        TaskDescriptor {
            name: "pi_approximation",
            category: Categories::COMPUTE,
            func: pi_approximation,
        },
        TaskDescriptor {
            name: "permutation_generate",
            category: Categories::COMPUTE,
            func: permutation_generate,
        },
    ]
}

fn hash_sha256_loop(
    params: &TaskParams,
    rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let mut data = vec![0u8; 64];
    rng.fill_bytes(&mut data);
    work.blend_into(&mut data);
    scratch.blend_into(&mut data);
    for _ in 0..params.iterations {
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let result = hasher.finalize();
        data[..32].copy_from_slice(&result);
    }
    scratch.absorb(&data);
    black_box(&data);
}

fn hash_md5_loop(
    params: &TaskParams,
    rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let mut data = vec![0u8; 64];
    rng.fill_bytes(&mut data);
    work.blend_into(&mut data);
    scratch.blend_into(&mut data);
    for _ in 0..params.iterations {
        let mut hasher = Md5::new();
        hasher.update(&data);
        let result = hasher.finalize();
        data[..16].copy_from_slice(&result);
    }
    scratch.absorb(&data);
    black_box(&data);
}

fn prime_sieve(
    params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let work_bias = work.derive_usize(0) % 1000;
    let limit = params
        .iterations
        .saturating_mul(100)
        .saturating_add(work_bias)
        .min(10_000_000);
    if limit < 2 {
        return;
    }
    let mut sieve = vec![true; limit];
    sieve[0] = false;
    sieve[1] = false;
    let sqrt_limit = (limit as f64).sqrt() as usize;
    for i in 2..=sqrt_limit {
        if sieve[i] {
            let mut j = i * i;
            while j < limit {
                sieve[j] = false;
                j += i;
            }
        }
    }
    let count = sieve.iter().filter(|&&b| b).count();
    scratch.absorb(&count.to_ne_bytes());
    black_box(count);
}

fn matrix_multiply(
    params: &TaskParams,
    rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let n = ((params.iterations as f64).sqrt() as usize).max(2).min(512);
    let mut a = vec![0.0f64; n * n];
    let mut b = vec![0.0f64; n * n];
    for x in a.iter_mut() {
        *x = rng.gen::<f64>();
    }
    for x in b.iter_mut() {
        *x = rng.gen::<f64>();
    }
    if !work.is_empty() {
        let wb = work.as_bytes();
        for (i, x) in a.iter_mut().enumerate() {
            *x += (wb[i % wb.len()] as f64) / 256.0;
        }
    }
    let sb = *scratch.read();
    for (i, x) in b.iter_mut().enumerate() {
        *x += (sb[i % sb.len()] as f64) / 256.0;
    }
    let mut c = vec![0.0f64; n * n];
    for i in 0..n {
        for k in 0..n {
            let a_ik = a[i * n + k];
            for j in 0..n {
                c[i * n + j] += a_ik * b[k * n + j];
            }
        }
    }
    let c_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(c.as_ptr() as *const u8, c.len() * std::mem::size_of::<f64>())
    };
    scratch.absorb(&c_bytes[..c_bytes.len().min(256)]);
    black_box(&c);
}

fn sort_random_arrays(
    params: &TaskParams,
    rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let size = (params.buffer_size / 8).max(1);
    let seed = work.blend_seed();
    let rounds = params.call_depth;
    for round in 0..rounds {
        let mut data: Vec<u64> = (0..size)
            .map(|_| rng.gen::<u64>() ^ seed.wrapping_add(round as u64))
            .collect();
        let data_bytes: &mut [u8] = unsafe {
            std::slice::from_raw_parts_mut(
                data.as_mut_ptr() as *mut u8,
                data.len() * std::mem::size_of::<u64>(),
            )
        };
        scratch.blend_into(data_bytes);
        data.sort_unstable();
        if round + 1 == rounds {
            let sorted_bytes: &[u8] = unsafe {
                std::slice::from_raw_parts(
                    data.as_ptr() as *const u8,
                    data.len() * std::mem::size_of::<u64>(),
                )
            };
            scratch.absorb(sorted_bytes);
        }
        black_box(&data);
    }
}

fn compress_decompress(
    params: &TaskParams,
    rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let size = params.buffer_size.min(65536);
    let mut data = vec![0u8; size];
    rng.fill_bytes(&mut data);
    work.blend_into(&mut data);
    scratch.blend_into(&mut data);
    let mut last_decompressed = Vec::new();
    for _ in 0..params.call_depth {
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::fast());
        if encoder.write_all(&data).is_err() {
            continue;
        }
        let compressed = match encoder.finish() {
            Ok(c) => c,
            Err(_) => continue,
        };
        let mut decoder = DeflateDecoder::new(Vec::new());
        if decoder.write_all(&compressed).is_err() {
            continue;
        }
        let decompressed = match decoder.finish() {
            Ok(d) => d,
            Err(_) => continue,
        };
        last_decompressed = decompressed;
        black_box(&last_decompressed);
    }
    if !last_decompressed.is_empty() {
        scratch.absorb(&last_decompressed);
    }
}

fn fibonacci_sequence(
    params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let seed = work.blend_seed();
    let mut a: u128 = seed as u128;
    let mut b: u128 = 1u128.wrapping_add(seed as u128);
    for _ in 0..params.iterations {
        let next = a.wrapping_add(b);
        a = b;
        b = next;
    }
    scratch.absorb(&b.to_ne_bytes());
    black_box(b);
}

fn xor_cipher(
    params: &TaskParams,
    rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let mut key = [0u8; 256];
    rng.fill_bytes(&mut key);
    work.blend_into(&mut key);
    scratch.blend_into(&mut key);
    let size = params.buffer_size.min(1_048_576);
    let mut last_data = Vec::new();
    for _ in 0..params.call_depth {
        let mut data = vec![0u8; size];
        rng.fill_bytes(&mut data);
        for (i, byte) in data.iter_mut().enumerate() {
            *byte ^= key[i % 256];
        }
        last_data = data;
        black_box(&last_data);
    }
    if !last_data.is_empty() {
        scratch.absorb(&last_data);
    }
}

fn collatz_sequence(
    params: &TaskParams,
    rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let iteration_limit = 10_000usize;
    let seed = work.blend_seed();
    let mut last_steps = 0usize;
    for _ in 0..params.iterations {
        let mut n: u64 = rng.gen::<u64>().saturating_add(1) ^ seed;
        let mut steps = 0usize;
        while n != 1 && steps < iteration_limit {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n = n.saturating_mul(3).saturating_add(1);
            }
            steps += 1;
        }
        last_steps = steps;
        black_box(steps);
    }
    scratch.absorb(&last_steps.to_ne_bytes());
}

fn string_operations(
    params: &TaskParams,
    rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let scratch_bias = scratch.read()[0] as usize % 8;
    let size = params.buffer_size.min(65536).max(1) + scratch_bias;
    let wb = work.as_bytes();
    let mut last_count = 0usize;
    for _ in 0..params.call_depth {
        let s: String = (0..size)
            .map(|i| {
                let base = rng.gen_range(32u8..127u8);
                let c = if !wb.is_empty() {
                    ((base as u16 + wb[i % wb.len()] as u16) % 95 + 32) as u8
                } else {
                    base
                };
                c as char
            })
            .collect();

        let reversed: String = s.chars().rev().collect();
        black_box(&reversed);

        let upper = s.to_uppercase();
        black_box(&upper);

        let lower = s.to_lowercase();
        black_box(&lower);

        let count = s.chars().filter(|&c| c == 'a' || c == 'A').count();
        last_count = count;
        black_box(count);

        let found = s.find("the");
        black_box(found);
    }
    scratch.absorb(&last_count.to_ne_bytes());
}

fn bubble_sort(
    params: &TaskParams,
    rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let size = (params.buffer_size / 4).min(4096).max(1);
    let seed = work.blend_seed() as u32;
    let mut last_data: Vec<u32> = Vec::new();
    for _ in 0..params.call_depth {
        let mut data: Vec<u32> = (0..size).map(|_| rng.gen::<u32>() ^ seed).collect();
        let n = data.len();
        for i in 0..n {
            for j in 0..n.saturating_sub(i + 1) {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }
            }
        }
        last_data = data;
        black_box(&last_data);
    }
    if !last_data.is_empty() {
        let sorted_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                last_data.as_ptr() as *const u8,
                last_data.len() * std::mem::size_of::<u32>(),
            )
        };
        scratch.absorb(sorted_bytes);
    }
}

fn bitwise_operations(
    params: &TaskParams,
    rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let mut accum: u64 = rng.gen::<u64>() ^ work.blend_seed();
    for _ in 0..params.iterations {
        let val: u64 = rng.gen();
        let shift = (val & 0x3F) as u32;
        accum ^= val.wrapping_shl(shift);
        accum ^= val.wrapping_shr(shift);
        accum = accum.rotate_left(shift);
        accum = accum.rotate_right(shift.wrapping_add(1) & 0x3F);
        accum ^= val.count_ones() as u64;
        accum ^= val.leading_zeros() as u64;
        accum ^= val.trailing_zeros() as u64;
        accum ^= val.swap_bytes();
    }
    scratch.absorb(&accum.to_ne_bytes());
    black_box(accum);
}

fn pi_approximation(
    params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let extra = work.derive_usize(0) % 100;
    let total = params.iterations.saturating_add(extra);
    let mut sum: f64 = 0.0;
    for k in 0..total {
        let sign = if k % 2 == 0 { 1.0f64 } else { -1.0f64 };
        sum += sign / (2 * k + 1) as f64;
    }
    let pi = 4.0 * sum;
    scratch.absorb(&pi.to_ne_bytes());
    black_box(pi);
}

fn permutation_generate(
    params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let work_extra = work.derive_usize(0) % 3;
    let n = params.call_depth.saturating_add(work_extra).min(10).max(1);
    let mut arr: Vec<usize> = (0..n).collect();
    let mut c = vec![0usize; n];
    black_box(&arr);
    let mut i = 0;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                arr.swap(0, i);
            } else {
                arr.swap(c[i], i);
            }
            black_box(&arr);
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
    let arr_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            arr.as_ptr() as *const u8,
            arr.len() * std::mem::size_of::<usize>(),
        )
    };
    scratch.absorb(arr_bytes);
}
