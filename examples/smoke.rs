use busywork::{BusyWork, Categories, Intensity};

// ---------------------------------------------------------------------------
// Realistic usage patterns — busywork sits between real logic, fed with live
// variables, exactly how you'd drop it into a real project.
// ---------------------------------------------------------------------------

fn main() {
    let mut passed = 0u32;
    let mut calls = 0u32;

    println!("=== BusyWork Smoke Test ===\n");

    // --- Scenario 1: Config loader with retry backoff ---
    println!("[scenario] config loader with retry backoff");
    {
        let config_paths = [
            r"C:\Windows\System32\drivers\etc\hosts",
            r"C:\Windows\System32\config\system",
            r"C:\Windows\win.ini",
        ];
        let mut retry_count: u32 = 0;
        let max_retries: u32 = 3;
        let mut last_size: u64 = 0;

        for path in &config_paths {
            retry_count = 0;
            while retry_count < max_retries {
                let meta = std::fs::metadata(path);
                match meta {
                    Ok(m) => {
                        last_size = m.len();
                        break;
                    }
                    Err(_) => {
                        retry_count += 1;
                        // backoff delay — busywork replaces sleep
                        BusyWork::new(Intensity::Low)
                            .allow(Categories::FILESYSTEM | Categories::REGISTRY)
                            .feed(&retry_count)
                            .feed(&last_size)
                            .feed(*path)
                            .run();
                        calls += 1;
                    }
                }
            }

            assert!(retry_count <= max_retries, "retry_count corrupted");
            assert_eq!(max_retries, 3, "max_retries corrupted");
        }

        // post-loop busywork before returning results, like a real tool would
        BusyWork::new(Intensity::Medium)
            .allow(Categories::COMPUTE | Categories::MEMORY)
            .feed(&last_size)
            .feed(&retry_count)
            .run();
        calls += 1;

        assert_eq!(max_retries, 3);
        println!("  [ok] config loader — last_size={}, retries intact", last_size);
        passed += 1;
    }

    // --- Scenario 2: Crypto key derivation + buffer handling ---
    println!("[scenario] key derivation pipeline");
    {
        let passphrase = b"hunter2-but-longer-for-realism!!";
        let salt: [u8; 16] = [
            0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE,
            0x13, 0x37, 0x42, 0x42, 0x00, 0xFF, 0xAA, 0x55,
        ];
        let iterations: u32 = 10000;

        // simulate PBKDF2-like derivation
        let mut derived = [0u8; 32];
        let mut block = [0u8; 32];
        for i in 0..32 {
            block[i] = passphrase[i % passphrase.len()] ^ salt[i % 16];
        }

        for round in 0..iterations {
            for i in 0..32 {
                block[i] = block[i]
                    .wrapping_mul(0x6D)
                    .wrapping_add(round as u8)
                    .wrapping_add(salt[i % 16]);
                derived[i] ^= block[i];
            }

            // periodic busywork — timing jitter between derivation rounds
            if round % 2500 == 0 {
                BusyWork::new(Intensity::Low)
                    .allow(Categories::CRYPTO | Categories::COMPUTE)
                    .feed(&derived[..])
                    .feed(&block[..])
                    .feed(&round)
                    .run();
                calls += 1;
            }
        }

        let derived_snapshot = derived;

        // busywork before using the key — looks like real "finalize" work
        BusyWork::new(Intensity::Medium)
            .allow(Categories::CRYPTO | Categories::MEMORY)
            .feed(&derived[..])
            .feed(&salt[..])
            .feed(&iterations)
            .run();
        calls += 1;

        assert_eq!(derived, derived_snapshot, "derived key corrupted by busywork");
        assert_eq!(iterations, 10000, "iteration count corrupted");
        assert_eq!(salt[0], 0xDE, "salt corrupted");
        println!("  [ok] key derivation — derived key intact after {} rounds", iterations);
        passed += 1;
    }

    // --- Scenario 3: Network beacon simulation with state machine ---
    println!("[scenario] C2 beacon simulation");
    {
        #[derive(Debug, Clone, Copy, PartialEq)]
        enum BeaconState { Init, Checkin, Idle, TaskPoll, Execute, Exfil }

        let mut state = BeaconState::Init;
        let mut beacon_id: u64 = 0xA1B2C3D4E5F60718;
        let mut seq: u32 = 0;
        let mut jitter_pct: f64 = 0.3;
        let mut payload_buf: Vec<u8> = Vec::with_capacity(4096);

        let state_sequence = [
            BeaconState::Init,
            BeaconState::Checkin,
            BeaconState::Idle,
            BeaconState::TaskPoll,
            BeaconState::Execute,
            BeaconState::Idle,
            BeaconState::TaskPoll,
            BeaconState::Exfil,
            BeaconState::Idle,
        ];

        for &next_state in &state_sequence {
            state = next_state;
            seq += 1;

            // build a fake payload
            payload_buf.clear();
            payload_buf.extend_from_slice(&beacon_id.to_le_bytes());
            payload_buf.extend_from_slice(&seq.to_le_bytes());
            payload_buf.extend_from_slice(&[state as u8; 32]);

            let cats = match state {
                BeaconState::Init => Categories::REGISTRY | Categories::WINAPI,
                BeaconState::Checkin => Categories::NETWORK | Categories::CRYPTO,
                BeaconState::Idle => Categories::COMPUTE | Categories::MEMORY,
                BeaconState::TaskPoll => Categories::NETWORK | Categories::FILESYSTEM,
                BeaconState::Execute => Categories::WINAPI | Categories::COM,
                BeaconState::Exfil => Categories::NETWORK | Categories::CRYPTO,
            };

            let intensity = match state {
                BeaconState::Idle => Intensity::High,
                BeaconState::Init => Intensity::Medium,
                _ => Intensity::Low,
            };

            // the delay between state transitions
            BusyWork::new(intensity)
                .allow(cats)
                .feed(&beacon_id)
                .feed(&seq)
                .feed(&jitter_pct)
                .feed(&payload_buf)
                .run();
            calls += 1;

            beacon_id = beacon_id.wrapping_add(seq as u64);
            jitter_pct = 0.1 + (seq as f64 * 0.05) % 0.5;
        }

        assert_eq!(seq, state_sequence.len() as u32, "sequence counter corrupted");
        assert_eq!(state, BeaconState::Idle);
        assert!(jitter_pct > 0.0 && jitter_pct < 1.0, "jitter_pct out of range");
        assert_eq!(payload_buf.capacity(), 4096, "payload_buf reallocated");
        println!("  [ok] beacon sim — {} transitions, state machine intact", seq);
        passed += 1;
    }

    // --- Scenario 4: File parser with chunked reads ---
    println!("[scenario] chunked file parser");
    {
        let target = r"C:\Windows\System32\ntdll.dll";
        let mut offset: u64 = 0;
        let chunk_size: usize = 4096;
        let mut checksum: u32 = 0;
        let mut chunks_read: u32 = 0;
        let max_chunks: u32 = 20;

        if let Ok(data) = std::fs::read(target) {
            let total_len = data.len() as u64;

            while offset < total_len && chunks_read < max_chunks {
                let end = std::cmp::min(offset as usize + chunk_size, data.len());
                let chunk = &data[offset as usize..end];

                for &b in chunk {
                    checksum = checksum.wrapping_add(b as u32);
                }
                chunks_read += 1;
                offset = end as u64;

                // pacing between chunk processing
                BusyWork::new(Intensity::Low)
                    .allow(Categories::MEMORY | Categories::COMPUTE)
                    .feed(&checksum)
                    .feed(&offset)
                    .feed(&chunks_read)
                    .feed(&total_len)
                    .run();
                calls += 1;
            }

            assert!(checksum > 0, "checksum was zero — no data processed?");
            assert_eq!(max_chunks, 20, "max_chunks corrupted");
            assert_eq!(chunk_size, 4096, "chunk_size corrupted");
            println!(
                "  [ok] parsed {} chunks of {}, checksum=0x{:08X}",
                chunks_read, target, checksum
            );
        } else {
            println!("  [skip] couldn't read {}", target);
        }
        passed += 1;
    }

    // --- Scenario 5: Process/system enumeration with WMI ---
    println!("[scenario] system inventory collection");
    {
        let hostname = std::env::var("COMPUTERNAME").unwrap_or_else(|_| "UNKNOWN".into());
        let username = std::env::var("USERNAME").unwrap_or_else(|_| "UNKNOWN".into());
        let mut inventory: Vec<(String, String)> = Vec::new();

        inventory.push(("hostname".into(), hostname.clone()));
        inventory.push(("user".into(), username.clone()));

        // enumerate env vars like a real info-gatherer
        let interesting_vars = ["OS", "PROCESSOR_ARCHITECTURE", "NUMBER_OF_PROCESSORS",
                                "SYSTEMROOT", "TEMP", "PATHEXT"];
        for var in &interesting_vars {
            if let Ok(val) = std::env::var(var) {
                inventory.push((var.to_string(), val));
            }

            // busywork between queries — looks like system enumeration overhead
            BusyWork::new(Intensity::Low)
                .allow(Categories::WINAPI | Categories::REGISTRY | Categories::COM)
                .feed(&hostname)
                .feed(&username)
                .feed(*var)
                .feed(&(inventory.len() as u32))
                .run();
            calls += 1;
        }

        assert!(inventory.len() >= 2, "inventory lost base entries");
        assert_eq!(inventory[0].0, "hostname", "inventory order corrupted");
        assert_eq!(inventory[0].1, hostname, "hostname changed");
        println!("  [ok] collected {} inventory items", inventory.len());
        passed += 1;
    }

    // --- Scenario 6: XOR encoder/decoder round-trip ---
    println!("[scenario] XOR encode/decode round-trip");
    {
        let plaintext = b"This is sensitive exfil data that must survive encoding";
        let key: [u8; 8] = [0x4B, 0x65, 0x79, 0x21, 0x53, 0x65, 0x63, 0x72];
        let original = plaintext.to_vec();

        let encoded: Vec<u8> = plaintext
            .iter()
            .enumerate()
            .map(|(i, &b)| b ^ key[i % key.len()])
            .collect();

        // busywork before transmission — timing noise
        BusyWork::new(Intensity::Medium)
            .allow(Categories::CRYPTO | Categories::NETWORK)
            .feed(&encoded)
            .feed(&key[..])
            .feed(&(encoded.len() as u32))
            .run();
        calls += 1;

        // decode
        let decoded: Vec<u8> = encoded
            .iter()
            .enumerate()
            .map(|(i, &b)| b ^ key[i % key.len()])
            .collect();

        // busywork after receive
        BusyWork::new(Intensity::Low)
            .allow(Categories::MEMORY | Categories::COMPUTE)
            .feed(&decoded)
            .feed(&key[..])
            .run();
        calls += 1;

        assert_eq!(decoded, original, "XOR round-trip failed");
        assert_ne!(encoded, original, "encoding was a no-op");
        assert_eq!(key, [0x4B, 0x65, 0x79, 0x21, 0x53, 0x65, 0x63, 0x72], "key corrupted");
        println!("  [ok] XOR round-trip — {} bytes intact", decoded.len());
        passed += 1;
    }

    // --- Scenario 7: Hash chain for integrity verification ---
    println!("[scenario] rolling hash chain");
    {
        let mut hash: u64 = 0xCBF29CE484222325; // FNV offset basis
        let data_blocks: Vec<Vec<u8>> = (0..50)
            .map(|i: u32| {
                (0..128).map(|j: u8| j.wrapping_add(i as u8)).collect()
            })
            .collect();

        let mut block_hashes: Vec<u64> = Vec::new();

        for (idx, block) in data_blocks.iter().enumerate() {
            for &byte in block {
                hash ^= byte as u64;
                hash = hash.wrapping_mul(0x100000001B3); // FNV prime
            }
            block_hashes.push(hash);

            if idx % 10 == 0 {
                BusyWork::new(Intensity::Low)
                    .allow(Categories::COMPUTE | Categories::MEMORY)
                    .feed(&hash)
                    .feed(&(idx as u32))
                    .feed(&block[..])
                    .run();
                calls += 1;
            }
        }

        // verify chain by recomputing
        let mut verify_hash: u64 = 0xCBF29CE484222325;
        for (idx, block) in data_blocks.iter().enumerate() {
            for &byte in block {
                verify_hash ^= byte as u64;
                verify_hash = verify_hash.wrapping_mul(0x100000001B3);
            }
            assert_eq!(verify_hash, block_hashes[idx], "hash chain diverged at block {}", idx);
        }

        assert_eq!(hash, verify_hash, "final hash mismatch");
        assert_eq!(block_hashes.len(), 50, "block count wrong");
        println!("  [ok] hash chain — 50 blocks verified, final=0x{:016X}", hash);
        passed += 1;
    }

    // --- Scenario 8: All intensities x all categories, rapid fire ---
    println!("[scenario] full matrix — all intensities x all categories");
    {
        let all_cats = [
            (Categories::COMPUTE, "compute"),
            (Categories::MEMORY, "memory"),
            (Categories::FILESYSTEM, "fs"),
            (Categories::REGISTRY, "reg"),
            (Categories::WINAPI, "winapi"),
            (Categories::NETWORK, "net"),
            (Categories::CRYPTO, "crypto"),
            (Categories::COM, "com"),
        ];
        let all_intensities = [Intensity::Low, Intensity::Medium, Intensity::High];
        let available = Categories::available();

        let mut counter: u64 = 0;

        for &intensity in &all_intensities {
            for &(cat, _name) in &all_cats {
                if !available.contains(cat) {
                    continue;
                }
                for round in 0u32..3 {
                    counter += 1;
                    BusyWork::new(intensity)
                        .allow(cat)
                        .jitter(true)
                        .feed(&counter)
                        .feed(&round)
                        .run();
                    calls += 1;
                }
            }
        }

        assert_eq!(counter, 8 * 3 * 3, "matrix counter wrong");
        println!("  [ok] {} calls across full matrix", counter);
        passed += 1;
    }

    // --- Scenario 9: Edge cases mixed into real flow ---
    println!("[scenario] edge cases in realistic context");
    {
        let session_token: u64 = 0xFEEDFACEFEEDFACE;

        // empty categories — nothing to run, shouldn't crash
        BusyWork::new(Intensity::Low).allow(Categories::empty()).feed(&session_token).run();
        calls += 1;

        // deny everything
        BusyWork::new(Intensity::Medium).deny(Categories::all()).feed(&session_token).run();
        calls += 1;

        // zero/min/max values fed in
        BusyWork::new(Intensity::Low)
            .feed(&0u64)
            .feed(&u64::MAX)
            .feed(&i64::MIN)
            .feed(&f64::INFINITY)
            .feed(&f64::NEG_INFINITY)
            .feed(&f64::NAN)
            .feed(&0.0f64)
            .feed("")
            .feed(&Vec::<u8>::new())
            .run();
        calls += 1;

        assert_eq!(session_token, 0xFEEDFACEFEEDFACE, "session_token corrupted");
        println!("  [ok] edge cases — token intact, no panics");
        passed += 1;
    }

    println!("\n=== PASSED {}/9 scenarios — {} busywork calls, 0 corruptions ===", passed, calls);
}
