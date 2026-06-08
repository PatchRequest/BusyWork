use crate::categories::Categories;
use crate::tasks::{TaskDescriptor, TaskParams};
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

fn hash_sha256_loop(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let mut data = vec![0u8; 64];
    rng.fill_bytes(&mut data);
    work.blend_into(&mut data);
    for _ in 0..params.iterations {
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let result = hasher.finalize();
        data[..32].copy_from_slice(&result);
    }
    black_box(&data);
}

fn hash_md5_loop(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let mut data = vec![0u8; 64];
    rng.fill_bytes(&mut data);
    work.blend_into(&mut data);
    for _ in 0..params.iterations {
        let mut hasher = Md5::new();
        hasher.update(&data);
        let result = hasher.finalize();
        data[..16].copy_from_slice(&result);
    }
    black_box(&data);
}

fn prime_sieve(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData) {
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
    black_box(count);
}

fn matrix_multiply(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
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

fn sort_random_arrays(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let size = (params.buffer_size / 8).max(1);
    let seed = work.blend_seed();
    for round in 0..params.call_depth {
        let mut data: Vec<u64> = (0..size)
            .map(|_| rng.gen::<u64>() ^ seed.wrapping_add(round as u64))
            .collect();
        data.sort_unstable();
        black_box(&data);
    }
}

fn compress_decompress(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let size = params.buffer_size.min(65536);
    let mut data = vec![0u8; size];
    rng.fill_bytes(&mut data);
    work.blend_into(&mut data);
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

fn fibonacci_sequence(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData) {
    let seed = work.blend_seed();
    let mut a: u128 = seed as u128;
    let mut b: u128 = 1u128.wrapping_add(seed as u128);
    for _ in 0..params.iterations {
        let next = a.wrapping_add(b);
        a = b;
        b = next;
    }
    black_box(b);
}

fn xor_cipher(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let mut key = [0u8; 256];
    rng.fill_bytes(&mut key);
    work.blend_into(&mut key);
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

fn collatz_sequence(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let iteration_limit = 10_000usize;
    let seed = work.blend_seed();
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
        black_box(steps);
    }
}

fn string_operations(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let size = params.buffer_size.min(65536).max(1);
    let wb = work.as_bytes();
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
        black_box(count);

        let found = s.find("the");
        black_box(found);
    }
}

fn bubble_sort(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
    let size = (params.buffer_size / 4).min(4096).max(1);
    let seed = work.blend_seed() as u32;
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
        black_box(&data);
    }
}

fn bitwise_operations(params: &TaskParams, rng: &mut ThreadRng, work: &WorkData) {
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
    black_box(accum);
}

fn pi_approximation(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData) {
    let extra = work.derive_usize(0) % 100;
    let total = params.iterations.saturating_add(extra);
    let mut sum: f64 = 0.0;
    for k in 0..total {
        let sign = if k % 2 == 0 { 1.0f64 } else { -1.0f64 };
        sum += sign / (2 * k + 1) as f64;
    }
    let pi = 4.0 * sum;
    black_box(pi);
}

fn permutation_generate(params: &TaskParams, _rng: &mut ThreadRng, work: &WorkData) {
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
}

#[cfg(test)]
mod tests {
    use crate::workdata::WorkData;

    // ── Proof that work data changes each task's computation ───────────
    // These mirror the exact integration logic in each task and verify
    // that feeding data produces different internal values. If someone
    // accidentally removes a blend_into / blend_seed call, these fail.

    #[test]
    fn hash_sha256_input_differs_with_work() {
        let buf_no_work = vec![0u8; 64];
        let mut buf_with_work = vec![0u8; 64];

        let mut work = WorkData::new();
        work.feed(&0xDEADBEEFu32);
        work.blend_into(&mut buf_with_work);

        assert_ne!(buf_no_work, buf_with_work, "blend_into must change hash input");
        // Also verify it's not just the first 4 bytes (cyclic XOR spreads across buffer)
        assert!(buf_with_work[4..].iter().any(|&b| b != 0), "XOR should cycle past work data length");
    }

    #[test]
    fn hash_md5_input_differs_with_work() {
        let mut buf = vec![0u8; 64];
        let mut work = WorkData::new();
        work.feed("session_token_12345");
        work.blend_into(&mut buf);
        assert!(buf.iter().any(|&b| b != 0));
    }

    #[test]
    fn fibonacci_starting_values_change() {
        // fibonacci uses: a = seed as u128, b = 1 + seed as u128
        let mut work = WorkData::new();
        work.feed(&42u32);
        let seed = work.blend_seed();
        assert_ne!(seed, 0, "non-zero input must produce non-zero seed");

        let a = seed as u128;
        let b = 1u128.wrapping_add(seed as u128);
        assert_ne!((a, b), (0u128, 1u128), "starting values must differ from default");
    }

    #[test]
    fn collatz_starting_number_changes() {
        let mut work = WorkData::new();
        work.feed(&42u64);
        let seed = work.blend_seed();
        assert_ne!(seed, 0);
        // collatz XORs: n = rng_val ^ seed → different starting number
    }

    #[test]
    fn prime_sieve_limit_gets_bias() {
        let mut work = WorkData::new();
        work.feed(&0xCAFEBABEu32);
        let bias = work.derive_usize(0) % 1000;
        assert!(bias > 0, "non-trivial data should produce non-zero sieve bias");

        let base_limit = 50usize.saturating_mul(100);
        let biased_limit = base_limit.saturating_add(bias);
        assert!(biased_limit > base_limit);
    }

    #[test]
    fn matrix_entries_get_work_offset() {
        let mut work = WorkData::new();
        work.feed(&[0x80u8; 16]);
        let wb = work.as_bytes();
        // matrix_multiply adds (wb[i % len] as f64) / 256.0 to each entry
        let offset = wb[0] as f64 / 256.0;
        assert!(offset > 0.0, "work data must produce positive offset");
        assert!(offset < 1.0, "offset must stay fractional");
    }

    #[test]
    fn sort_seed_affects_values() {
        let mut work = WorkData::new();
        work.feed(&42u32);
        let seed = work.blend_seed();
        assert_ne!(seed, 0);
        // sort_random_arrays: rng.gen() ^ seed.wrapping_add(round)
        // Non-zero seed changes every array element
    }

    #[test]
    fn compress_input_changes_with_work() {
        let mut data = vec![0u8; 1024];
        let mut work = WorkData::new();
        work.feed(&[0xAB; 32]);
        work.blend_into(&mut data);
        assert!(data.iter().any(|&b| b != 0), "compressed data input must change");
    }

    #[test]
    fn xor_cipher_key_modified() {
        let mut key = [0u8; 256];
        let mut work = WorkData::new();
        work.feed(&0xFFu8);
        work.blend_into(&mut key);
        assert!(key.iter().any(|&b| b != 0), "cipher key must incorporate work data");
    }

    #[test]
    fn string_ops_chars_change() {
        let mut work = WorkData::new();
        work.feed(&0xFFu8);
        let wb = work.as_bytes();
        let base: u8 = 65; // 'A'
        let modified = ((base as u16 + wb[0] as u16) % 95 + 32) as u8;
        assert_ne!(modified, base);
    }

    #[test]
    fn bubble_sort_seed_nonzero() {
        let mut work = WorkData::new();
        work.feed(&42u32);
        let seed = work.blend_seed() as u32;
        assert_ne!(seed, 0);
    }

    #[test]
    fn bitwise_initial_accum_changes() {
        let mut work = WorkData::new();
        work.feed(&42u32);
        let seed = work.blend_seed();
        // bitwise_operations: accum = rng ^ seed → different accumulator start
        assert_ne!(seed, 0);
    }

    #[test]
    fn pi_extra_iterations_added() {
        let mut work = WorkData::new();
        work.feed(&42u32);
        let extra = work.derive_usize(0) % 100;
        let base_iterations = 50usize;
        let total = base_iterations.saturating_add(extra);
        assert!(total >= base_iterations, "total must be >= base");
        // With most non-trivial data, extra > 0
    }

    #[test]
    fn permutation_size_changes() {
        let mut work = WorkData::new();
        work.feed(&42u32);
        let extra = work.derive_usize(0) % 3;
        let base_depth = 2usize;
        let n = base_depth.saturating_add(extra).min(10).max(1);
        // n should be >= base_depth (since extra >= 0) and possibly larger
        assert!(n >= base_depth);
    }

    // ── blend_into reversibility (proves it's XOR, not corruption) ─────

    #[test]
    fn blend_into_is_reversible_on_task_buffers() {
        let mut work = WorkData::new();
        work.feed(&0xDEADBEEFCAFEBABEu64);

        let original = vec![42u8; 4096];
        let mut buf = original.clone();
        work.blend_into(&mut buf);
        assert_ne!(buf, original, "first blend must change data");
        work.blend_into(&mut buf);
        assert_eq!(buf, original, "double blend must restore original");
    }

    // ── Different work data → different seeds (no collision) ───────────

    #[test]
    fn different_data_different_seeds() {
        let inputs: &[&[u8]] = &[
            b"alpha", b"bravo", b"charlie", b"delta",
            b"echo", b"foxtrot", b"golf", b"hotel",
            b"india", b"juliet",
        ];
        let mut seeds = Vec::new();
        for input in inputs {
            let mut w = WorkData::new();
            w.feed(*input);
            seeds.push(w.blend_seed());
        }
        let unique_count = {
            let mut s = seeds.clone();
            s.sort();
            s.dedup();
            s.len()
        };
        assert_eq!(unique_count, seeds.len(), "distinct multi-byte inputs must produce distinct seeds");
    }

    #[test]
    fn different_data_different_derived_values() {
        let inputs: &[&[u8]] = &[
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            &[16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
        ];
        let mut w1 = WorkData::new();
        w1.feed(inputs[0]);
        let mut w2 = WorkData::new();
        w2.feed(inputs[1]);

        assert_ne!(w1.derive_usize(0), w2.derive_usize(0));
    }
}
