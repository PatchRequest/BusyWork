# BusyWork

A Rust library that replaces `sleep()` with real, varied work. Every call executes a **completely different code path** — random category, random task, random iteration counts. Breaks static and dynamic analysis pattern matching.

**Zero time objects** in the library binary. No `Duration`, no `Instant`, no `SystemTime`. Intensity levels control work volume through hardcoded iteration counts, buffer sizes, and call depths.

## Usage

```rust
use busywork::{busywork, busywork_with, BusyWork, Categories, Intensity};

// Simple — random tasks across all categories
busywork(Intensity::Medium);

// Pick specific categories
busywork_with(Intensity::High, Categories::COMPUTE | Categories::WINAPI);

// Builder for full control
BusyWork::new(Intensity::Ultra)
    .allow(Categories::COMPUTE | Categories::FILESYSTEM)
    .deny(Categories::NETWORK)
    .jitter(true)
    .run();
```

Drop it in any loop — every iteration looks different:

```rust
loop {
    do_real_work();
    busywork(Intensity::Medium); // different execution path every time
}
```

## Intensity Levels

No timers — just work volume:

| Level    | Tasks/call | Iterations | Buffer size | Call depth |
|----------|------------|------------|-------------|------------|
| `Low`    | 2          | 50         | 1 KB        | 2          |
| `Medium` | 5          | 500        | 16 KB       | 4          |
| `High`   | 10         | 5,000      | 256 KB      | 8          |
| `Ultra`  | 20         | 50,000     | 1 MB        | 16         |

Jitter (on by default) randomizes all parameters by ±30%, so two consecutive calls at the same intensity produce different instruction traces.

## Categories — 76 Tasks

### COMPUTE (14 tasks)
SHA-256 hash chains, MD5 hash chains, prime sieve (Eratosthenes), matrix multiplication, array sorting, deflate compress/decompress, Fibonacci sequence, XOR cipher rounds, Collatz conjecture, string operations, bubble sort, bitwise operations, pi approximation (Leibniz), permutation generation (Heap's algorithm).

### MEMORY (10 tasks)
Alloc/touch/free pages, memcpy chains, sort random data, pattern fill & verify, heap fragmentation (many small allocs), ring buffer simulation, repeated binary search, buffer reversal, buffer interleaving, scatter/gather access patterns.

### FILESYSTEM (12 tasks)
Enumerate System32, temp dir, Program Files, fonts, drivers, prefetch, logs, user profile. Stat system files & DLLs. Read hosts, services, win.ini, system.ini. All **read-only** — no files created or modified.

### REGISTRY (10 tasks)
Read installed software, system info (ProductName, CurrentBuild), services, timezone, environment variables, network config (TCP/IP parameters), CPU hardware info, font list, startup programs (Run keys), file associations (HKCR). All **KEY_READ** — no writes.

### WINAPI (16 tasks)
Enumerate windows (EnumWindows + GetWindowTextW), enumerate processes (ToolHelp32 snapshot), system info (GetSystemInfo, GlobalMemoryStatusEx), clipboard read, system metrics (10 indices), foreground window, cursor position, desktop window, logical drives + drive types, volume info, disk free space, FindFirstFile/FindNextFile, module handles (12 DLLs), VirtualQuery memory walk, system/Windows directories, process/thread IDs.

### NETWORK (7 tasks)
DNS lookups (24 hosts), HTTP GET (11 endpoints), NTP queries (7 servers), HTTP HEAD requests, TCP connect probes (10 host:port targets), DNS with varied ports, HTTP POST/PUT to echo endpoints. Socket timeouts via raw `setsockopt` — no `Duration` type.

### CRYPTO (7 tasks)
BCryptGenRandom (system CSPRNG), BCrypt SHA-256 hashing, BCrypt SHA-512, BCrypt MD5, BCrypt SHA-1, AES-256 symmetric encrypt, RNG algorithm providers (RNG, FIPS186DSARNG, DUALECRNG).

## Benchmarks

Measured on Windows 11, debug build, 5 runs each:

### By Intensity (all categories)

| Level    | Min        | Avg        | Max         |
|----------|------------|------------|-------------|
| `Low`    | 0.10 ms    | 0.19 ms    | 0.33 ms     |
| `Medium` | 3.26 ms    | 941 ms     | 3,064 ms    |
| `High`   | 133 ms     | 1,855 ms   | 6,322 ms    |
| `Ultra`  | 4,439 ms   | 58,450 ms  | 176,147 ms  |

The wide min/max spread is intentional — each call picks different tasks with different costs.

### By Category (Medium intensity)

| Category   | Min       | Avg       | Max        |
|------------|-----------|-----------|------------|
| COMPUTE    | 14.69 ms  | 499 ms    | 1,256 ms   |
| MEMORY     | 14.93 ms  | 123 ms    | 242 ms     |
| FILESYSTEM | 0.81 ms   | 21.2 ms   | 48.0 ms    |
| REGISTRY   | 0.93 ms   | 4.64 ms   | 17.8 ms    |
| WINAPI     | 0.28 ms   | 5.01 ms   | 19.1 ms    |
| NETWORK    | 1,188 ms  | 3,350 ms  | 6,318 ms   |
| CRYPTO     | 5.47 ms   | 8.77 ms   | 12.8 ms    |

## Feature Flags

Each category is a cargo feature, all on by default:

```toml
[dependencies]
busywork = "0.1"

# Or cherry-pick:
busywork = { version = "0.1", default-features = false, features = ["cat-compute", "cat-memory"] }
```

| Feature          | Dependencies       | Description                  |
|------------------|--------------------|------------------------------|
| `cat-compute`    | sha2, md-5, flate2 | Pure CPU work                |
| `cat-memory`     | —                  | Allocation patterns          |
| `cat-filesystem` | —                  | Read-only filesystem I/O     |
| `cat-registry`   | windows            | Windows Registry reads       |
| `cat-winapi`     | windows            | Win32 API calls              |
| `cat-network`    | —                  | DNS, HTTP, NTP               |
| `cat-crypto`     | windows            | Windows CNG (BCrypt) crypto  |

## Design

- **Every call is unique** — random category selection, random task within category, random iteration counts
- **No deterministic signature** — even at the same intensity, call A and call B look nothing alike
- **Task registry rebuilt per call** — no stable global pointer, prevents devirtualization
- **Function pointers, not trait objects** — diverse call targets, no vtable pattern
- **`black_box` on all results** — prevents dead-code elimination
- **Windows-only** — full Win32 API access, no cross-platform abstraction layer

## Stats

```
76 tasks across 7 categories
2,688 lines of Rust
0 time objects in the library binary
```
