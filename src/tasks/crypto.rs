use crate::categories::Categories;
use crate::tasks::{TaskDescriptor, TaskParams};
use rand::rngs::ThreadRng;
use rand::RngCore;
use std::hint::black_box;
use windows::Win32::Security::Cryptography::*;

pub fn register() -> Vec<TaskDescriptor> {
    vec![
        TaskDescriptor {
            name: "bcrypt_gen_random",
            category: Categories::CRYPTO,
            func: bcrypt_gen_random,
        },
        TaskDescriptor {
            name: "bcrypt_hash",
            category: Categories::CRYPTO,
            func: bcrypt_hash,
        },
    ]
}

fn bcrypt_gen_random(params: &TaskParams, _rng: &mut ThreadRng) {
    let size = params.buffer_size.min(65536);
    let mut buffer = vec![0u8; size];
    for _ in 0..params.iterations.min(100) {
        unsafe {
            let _ = BCryptGenRandom(
                BCRYPT_ALG_HANDLE::default(),
                &mut buffer,
                BCRYPT_USE_SYSTEM_PREFERRED_RNG,
            );
        }
        black_box(&buffer);
    }
}

fn bcrypt_hash(params: &TaskParams, rng: &mut ThreadRng) {
    let data_size = params.buffer_size.min(65536);
    let mut data = vec![0u8; data_size];
    rng.fill_bytes(&mut data);

    unsafe {
        let mut alg = BCRYPT_ALG_HANDLE::default();
        let status = BCryptOpenAlgorithmProvider(
            &mut alg,
            windows::core::w!("SHA256"),
            None,
            BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS(0),
        );
        if status.0 != 0 {
            return;
        }

        for _ in 0..params.iterations.min(100) {
            let mut hash_handle = BCRYPT_HASH_HANDLE::default();
            let status = BCryptCreateHash(alg, &mut hash_handle, None, None, 0);
            if status.0 != 0 {
                continue;
            }

            let _ = BCryptHashData(hash_handle, &data, 0);

            let mut output = vec![0u8; 32]; // SHA-256 output
            let _ = BCryptFinishHash(hash_handle, &mut output, 0);

            black_box(&output);
            let _ = BCryptDestroyHash(hash_handle);
        }

        let _ = BCryptCloseAlgorithmProvider(alg, 0);
    }
}
