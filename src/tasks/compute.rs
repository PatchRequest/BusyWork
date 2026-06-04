use crate::categories::Categories;
use crate::tasks::{TaskDescriptor, TaskParams};
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

fn hash_sha256_loop(params: &TaskParams, rng: &mut ThreadRng) {
    let mut data = vec![0u8; 64];
    rng.fill_bytes(&mut data);
    for _ in 0..params.iterations {
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let result = hasher.finalize();
        data[..32].copy_from_slice(&result);
    }
    black_box(&data);
}

fn hash_md5_loop(params: &TaskParams, rng: &mut ThreadRng) {
    let mut data = vec![0u8; 64];
    rng.fill_bytes(&mut data);
    for _ in 0..params.iterations {
        let mut hasher = Md5::new();
        hasher.update(&data);
        let result = hasher.finalize();
        data[..16].copy_from_slice(&result);
    }
    black_box(&data);
}

fn prime_sieve(params: &TaskParams, _rng: &mut ThreadRng) {
    let limit = params.iterations.saturating_mul(100).min(10_000_000);
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
    black_box(count);
}

fn matrix_multiply(params: &TaskParams, rng: &mut ThreadRng) {
    let n = ((params.iterations as f64).sqrt() as usize).max(2).min(512);
    let mut a = vec![0.0f64; n * n];
    let mut b = vec![0.0f64; n * n];
    for x in a.iter_mut() {
        *x = rng.gen::<f64>();
    }
    for x in b.iter_mut() {
        *x = rng.gen::<f64>();
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
    black_box(&c);
}

fn sort_random_arrays(params: &TaskParams, rng: &mut ThreadRng) {
    let size = (params.buffer_size / 8).max(1);
    for _ in 0..params.call_depth {
        let mut data: Vec<u64> = (0..size).map(|_| rng.gen()).collect();
        data.sort_unstable();
        black_box(&data);
    }
}

fn compress_decompress(params: &TaskParams, rng: &mut ThreadRng) {
    let size = params.buffer_size.min(65536);
    let mut data = vec![0u8; size];
    rng.fill_bytes(&mut data);
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
        black_box(&decompressed);
    }
}

fn fibonacci_sequence(params: &TaskParams, _rng: &mut ThreadRng) {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    for _ in 0..params.iterations {
        let next = a.wrapping_add(b);
        a = b;
        b = next;
    }
    black_box(b);
}

fn xor_cipher(params: &TaskParams, rng: &mut ThreadRng) {
    let mut key = [0u8; 256];
    rng.fill_bytes(&mut key);
    let size = params.buffer_size.min(1_048_576);
    for _ in 0..params.call_depth {
        let mut data = vec![0u8; size];
        rng.fill_bytes(&mut data);
        for (i, byte) in data.iter_mut().enumerate() {
            *byte ^= key[i % 256];
        }
        black_box(&data);
    }
}

fn collatz_sequence(params: &TaskParams, rng: &mut ThreadRng) {
    let iteration_limit = 10_000usize;
    for _ in 0..params.iterations {
        let mut n: u64 = rng.gen::<u64>().saturating_add(1);
        let mut steps = 0usize;
        while n != 1 && steps < iteration_limit {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n = n.saturating_mul(3).saturating_add(1);
            }
            steps += 1;
        }
        black_box(steps);
    }
}

fn string_operations(params: &TaskParams, rng: &mut ThreadRng) {
    let size = params.buffer_size.min(65536).max(1);
    for _ in 0..params.call_depth {
        let s: String = (0..size)
            .map(|_| {
                let c = rng.gen_range(32u8..127u8);
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
        black_box(count);

        let found = s.find("the");
        black_box(found);
    }
}

fn bubble_sort(params: &TaskParams, rng: &mut ThreadRng) {
    let size = (params.buffer_size / 4).min(4096).max(1);
    for _ in 0..params.call_depth {
        let mut data: Vec<u32> = (0..size).map(|_| rng.gen()).collect();
        let n = data.len();
        for i in 0..n {
            for j in 0..n.saturating_sub(i + 1) {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }
            }
        }
        black_box(&data);
    }
}

fn bitwise_operations(params: &TaskParams, rng: &mut ThreadRng) {
    let mut accum: u64 = rng.gen();
    for _ in 0..params.iterations {
        let val: u64 = rng.gen();
        let shift = (val & 0x3F) as u32; // 0..63
        accum ^= val.wrapping_shl(shift);
        accum ^= val.wrapping_shr(shift);
        accum = accum.rotate_left(shift);
        accum = accum.rotate_right(shift.wrapping_add(1) & 0x3F);
        accum ^= val.count_ones() as u64;
        accum ^= val.leading_zeros() as u64;
        accum ^= val.trailing_zeros() as u64;
        accum ^= val.swap_bytes();
    }
    black_box(accum);
}

fn pi_approximation(params: &TaskParams, _rng: &mut ThreadRng) {
    let mut sum: f64 = 0.0;
    for k in 0..params.iterations {
        let sign = if k % 2 == 0 { 1.0f64 } else { -1.0f64 };
        sum += sign / (2 * k + 1) as f64;
    }
    let pi = 4.0 * sum;
    black_box(pi);
}

fn permutation_generate(params: &TaskParams, _rng: &mut ThreadRng) {
    let n = params.call_depth.min(10).max(1);
    let mut arr: Vec<usize> = (0..n).collect();
    // Heap's algorithm (iterative)
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
}
