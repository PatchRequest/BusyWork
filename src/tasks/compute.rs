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
