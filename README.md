# BusyWork

A Rust library that replaces `sleep()` with real, varied work to **evade behavioral pattern matching** by EDR, anti-cheat, and dynamic analysis systems.

Every call executes a **completely different code path** — random category, random task, random iteration counts. No two calls produce the same syscall sequence, instruction trace, or API call pattern.

## Why

`sleep()` is one of the most commonly hooked and monitored functions. EDR and anti-cheat systems flag predictable idle-then-act patterns as suspicious. Even if `sleep()` itself isn't hooked, the behavioral signature of "do X, idle N ms, do X again" is trivial to fingerprint.

BusyWork fills those gaps with **genuine, varied activity** — real syscalls, real computations, real I/O — that looks like normal application behavior under dynamic analysis. The execution profile changes on every call, defeating both signature-based and heuristic-based detection.

**Zero time objects** in the library binary. No `Duration`, no `Instant`, no `SystemTime`. Nothing for static analysis to latch onto as a timing mechanism.

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

## Anti-Detection Design

Every design decision optimizes for evasion:

| Technique | What it defeats |
|-----------|----------------|
| **Random task selection per call** | Behavioral sequence matching — no two calls produce the same API call order |
| **±30% jitter on all parameters** | Timing heuristics — iteration counts, buffer sizes, call depths all vary |
| **Task registry rebuilt per call** | Static analysis — no stable global function pointer table to fingerprint |
| **Function pointers, not trait objects** | Vtable signature detection — each task is a direct call to a unique address |
| **`black_box` on all results** | Dead-code elimination — compiler can't optimize away the work |
| **No time objects in binary** | String/import scanning — `Duration`, `Instant`, `SystemTime` are absent |
| **Real syscalls across categories** | Syscall sequence analysis — mixes NtReadFile, NtQueryKey, NtDeviceIoControl, etc. |
| **Read-only filesystem/registry ops** | Behavioral red flags — no writes, no creates, no deletes |
| **Legitimate API call targets** | API monitoring — calls the same APIs that normal applications use |

### What it looks like under analysis

A process calling `busywork(Medium)` in a loop produces traces like:

```
Call 1: RegOpenKeyEx → RegEnumKeyEx × 3 → RegCloseKey → SHA256 × 200 → ReadFile(hosts)
Call 2: EnumWindows → GetWindowText × 15 → GlobalMemoryStatusEx → sort(50KB)
Call 3: connect(httpbin.org:80) → send(GET) → recv → BCryptGenRandom × 8
Call 4: FindFirstFile(*.dll) × 40 → GetVolumeInformation → memcpy(16KB) × 300
```

No repeating pattern. Each call exercises different subsystems, different buffer sizes, different iteration counts. To an EDR, it looks like a normal application doing normal things.

## Stats

```
76 tasks across 7 categories
2,688 lines of Rust
0 time objects in the library binary
```
