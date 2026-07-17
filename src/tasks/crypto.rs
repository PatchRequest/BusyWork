use crate::categories::Categories;
use crate::tasks::{ScratchBuffer, TaskDescriptor, TaskParams};
use crate::workdata::WorkData;
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
        TaskDescriptor {
            name: "bcrypt_sha512",
            category: Categories::CRYPTO,
            func: bcrypt_sha512,
        },
        TaskDescriptor {
            name: "bcrypt_md5_hash",
            category: Categories::CRYPTO,
            func: bcrypt_md5_hash,
        },
        TaskDescriptor {
            name: "bcrypt_sha1_hash",
            category: Categories::CRYPTO,
            func: bcrypt_sha1_hash,
        },
        TaskDescriptor {
            name: "bcrypt_aes_encrypt",
            category: Categories::CRYPTO,
            func: bcrypt_aes_encrypt,
        },
        TaskDescriptor {
            name: "bcrypt_rng_algorithms",
            category: Categories::CRYPTO,
            func: bcrypt_rng_algorithms,
        },
    ]
}

fn bcrypt_gen_random(
    params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let size = params.buffer_size.min(65536);
    let mut buffer = vec![0u8; size];
    for _ in 0..params.iterations.min(100) {
        unsafe {
            let _ = BCryptGenRandom(
                None,
                &mut buffer,
                BCRYPT_USE_SYSTEM_PREFERRED_RNG,
            );
        }
        work.blend_into(&mut buffer);
        scratch.blend_into(&mut buffer);
        black_box(&buffer);
    }
    scratch.absorb(&buffer[..buffer.len().min(256)]);
}

fn bcrypt_hash(
    params: &TaskParams,
    rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let data_size = params.buffer_size.min(65536);
    let mut data = vec![0u8; data_size];
    rng.fill_bytes(&mut data);
    work.blend_into(&mut data);
    scratch.blend_into(&mut data);

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

        let mut output = vec![0u8; 32];

        for _ in 0..params.iterations.min(100) {
            let mut hash_handle = BCRYPT_HASH_HANDLE::default();
            let status = BCryptCreateHash(alg, &mut hash_handle, None, None, 0);
            if status.0 != 0 {
                continue;
            }

            let _ = BCryptHashData(hash_handle, &data, 0);
            let _ = BCryptFinishHash(hash_handle, &mut output, 0);

            black_box(&output);
            let _ = BCryptDestroyHash(hash_handle);
        }

        scratch.absorb(&output);
        let _ = BCryptCloseAlgorithmProvider(alg, 0);
    }
}

fn bcrypt_sha512(
    params: &TaskParams,
    rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let data_size = params.buffer_size.min(65536);
    let mut data = vec![0u8; data_size];
    rng.fill_bytes(&mut data);
    work.blend_into(&mut data);
    scratch.blend_into(&mut data);

    unsafe {
        let mut alg = BCRYPT_ALG_HANDLE::default();
        let status = BCryptOpenAlgorithmProvider(
            &mut alg,
            windows::core::w!("SHA512"),
            None,
            BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS(0),
        );
        if status.0 != 0 {
            return;
        }

        let mut output = vec![0u8; 64];

        for _ in 0..params.iterations.min(100) {
            let mut hash_handle = BCRYPT_HASH_HANDLE::default();
            let status = BCryptCreateHash(alg, &mut hash_handle, None, None, 0);
            if status.0 != 0 {
                continue;
            }

            let _ = BCryptHashData(hash_handle, &data, 0);
            let _ = BCryptFinishHash(hash_handle, &mut output, 0);

            black_box(&output);
            let _ = BCryptDestroyHash(hash_handle);
        }

        scratch.absorb(&output);
        let _ = BCryptCloseAlgorithmProvider(alg, 0);
    }
}

fn bcrypt_md5_hash(
    params: &TaskParams,
    rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let data_size = params.buffer_size.min(65536);
    let mut data = vec![0u8; data_size];
    rng.fill_bytes(&mut data);
    work.blend_into(&mut data);
    scratch.blend_into(&mut data);

    unsafe {
        let mut alg = BCRYPT_ALG_HANDLE::default();
        let status = BCryptOpenAlgorithmProvider(
            &mut alg,
            windows::core::w!("MD5"),
            None,
            BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS(0),
        );
        if status.0 != 0 {
            return;
        }

        let mut output = vec![0u8; 16];

        for _ in 0..params.iterations.min(100) {
            let mut hash_handle = BCRYPT_HASH_HANDLE::default();
            let status = BCryptCreateHash(alg, &mut hash_handle, None, None, 0);
            if status.0 != 0 {
                continue;
            }

            let _ = BCryptHashData(hash_handle, &data, 0);
            let _ = BCryptFinishHash(hash_handle, &mut output, 0);

            black_box(&output);
            let _ = BCryptDestroyHash(hash_handle);
        }

        scratch.absorb(&output);
        let _ = BCryptCloseAlgorithmProvider(alg, 0);
    }
}

fn bcrypt_sha1_hash(
    params: &TaskParams,
    rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let data_size = params.buffer_size.min(65536);
    let mut data = vec![0u8; data_size];
    rng.fill_bytes(&mut data);
    work.blend_into(&mut data);
    scratch.blend_into(&mut data);

    unsafe {
        let mut alg = BCRYPT_ALG_HANDLE::default();
        let status = BCryptOpenAlgorithmProvider(
            &mut alg,
            windows::core::w!("SHA1"),
            None,
            BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS(0),
        );
        if status.0 != 0 {
            return;
        }

        let mut output = vec![0u8; 20];

        for _ in 0..params.iterations.min(100) {
            let mut hash_handle = BCRYPT_HASH_HANDLE::default();
            let status = BCryptCreateHash(alg, &mut hash_handle, None, None, 0);
            if status.0 != 0 {
                continue;
            }

            let _ = BCryptHashData(hash_handle, &data, 0);
            let _ = BCryptFinishHash(hash_handle, &mut output, 0);

            black_box(&output);
            let _ = BCryptDestroyHash(hash_handle);
        }

        scratch.absorb(&output);
        let _ = BCryptCloseAlgorithmProvider(alg, 0);
    }
}

fn bcrypt_aes_encrypt(
    params: &TaskParams,
    rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let plaintext_size = params.buffer_size.min(4096);
    let mut plaintext = vec![0u8; plaintext_size];
    rng.fill_bytes(&mut plaintext);
    work.blend_into(&mut plaintext);
    scratch.blend_into(&mut plaintext);

    let mut key_bytes = [0u8; 32];
    rng.fill_bytes(&mut key_bytes);

    unsafe {
        let mut alg = BCRYPT_ALG_HANDLE::default();
        let status = BCryptOpenAlgorithmProvider(
            &mut alg,
            windows::core::w!("AES"),
            None,
            BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS(0),
        );
        if status.0 != 0 {
            return;
        }

        let mut output = vec![0u8; plaintext_size + 16];

        for _ in 0..params.call_depth.min(50) {
            let mut key_handle = BCRYPT_KEY_HANDLE::default();
            let status = BCryptGenerateSymmetricKey(
                alg,
                &mut key_handle,
                None,
                &key_bytes,
                0,
            );
            if status.0 != 0 {
                continue;
            }

            let mut bytes_written: u32 = 0;

            let _ = BCryptEncrypt(
                key_handle,
                Some(&plaintext),
                None,
                None,
                Some(&mut output),
                &mut bytes_written,
                BCRYPT_BLOCK_PADDING,
            );

            black_box(&output);
            black_box(bytes_written);

            let _ = BCryptDestroyKey(key_handle);
        }

        scratch.absorb(&output[..output.len().min(256)]);
        let _ = BCryptCloseAlgorithmProvider(alg, 0);
    }
}

fn bcrypt_rng_algorithms(
    params: &TaskParams,
    _rng: &mut ThreadRng,
    work: &WorkData,
    scratch: &mut ScratchBuffer,
) {
    let alg_ids = [
        windows::core::w!("RNG"),
        windows::core::w!("FIPS186DSARNG"),
        windows::core::w!("DUALECRNG"),
    ];

    let buf_size = params.buffer_size.min(4096);
    let mut buffer = vec![0u8; buf_size];

    for alg_id in &alg_ids {
        unsafe {
            let mut alg = BCRYPT_ALG_HANDLE::default();
            let status = BCryptOpenAlgorithmProvider(
                &mut alg,
                *alg_id,
                None,
                BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS(0),
            );
            if status.0 != 0 {
                continue;
            }

            for _ in 0..params.iterations.min(50) {
                let _ = BCryptGenRandom(Some(alg), &mut buffer, BCRYPTGENRANDOM_FLAGS(0));
                work.blend_into(&mut buffer);
                scratch.blend_into(&mut buffer);
                black_box(&buffer);
            }

            let _ = BCryptCloseAlgorithmProvider(alg, 0);
        }
    }

    scratch.absorb(&buffer[..buffer.len().min(256)]);
}
