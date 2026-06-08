# BusyWork

A Rust library that replaces `sleep()` with real, varied work to **evade behavioral pattern matching** by EDR, anti-cheat, and dynamic analysis systems.

Every call executes a **completely different code path** — random category, random task, random iteration counts. No two calls produce the same syscall sequence, instruction trace, or API call pattern.

## Why

`sleep()` is one of the most commonly hooked and monitored functions. EDR and anti-cheat systems flag predictable idle-then-act patterns as suspicious. Even if `sleep()` itself isn't hooked, the behavioral signature of "do X, idle N ms, do X again" is trivial to fingerprint.

BusyWork fills those gaps with **genuine, varied activity** — real syscalls, real computations, real I/O — that looks like normal application behavior under dynamic analysis. The execution profile changes on every call, defeating both signature-based and heuristic-based detection.

**Zero time objects** in the library binary. No `Duration`, no `Instant`, no `SystemTime`. Nothing for static analysis to latch onto as a timing mechanism.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
busywork = "0.1"
```

Or with only specific categories (faster compile, smaller binary):

```toml
[dependencies]
busywork = { version = "0.1", default-features = false, features = ["cat-compute", "cat-memory", "cat-winapi"] }
```

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

### Data Flow Integration

Busywork blocks can blend into surrounding code by processing **your variables**. Feed any data from the current scope — tasks will weave it into their control flow (hashing it, sorting it, using it as loop bounds), making the block indistinguishable from real data processing to static or dynamic analysis.

All data is **cloned on feed** — originals are never modified.

```rust
let session_id: u64 = 0xDEADBEEF;
let payload = vec![0x41u8; 1024];
let user_name = "admin";
let packet_count: u32 = 147;

BusyWork::new(Intensity::Medium)
    .allow(Categories::COMPUTE | Categories::MEMORY)
    .feed(&session_id)      // integers, floats, bools
    .feed(&payload)          // Vec<u8>, &[u8], [u8; N]
    .feed(user_name)         // &str, String
    .feed(&packet_count)     // unlimited parameters
    .run();
```

Without `.feed()`, tasks generate random input data — detectable as isolated noise blocks with no data dependency on the surrounding program. With `.feed()`, the busywork reads and processes the same variables as your real code. A data-flow analyzer cannot determine which computations are "real" and which are busywork.

**Works with custom types** — implement `FeedWork` for your own structs:

```rust
use busywork::FeedWork;

struct GameState { x: f32, y: f32, hp: u32 }

impl FeedWork for GameState {
    fn write_work_bytes(&self, out: &mut Vec<u8>) {
        self.x.write_work_bytes(out);
        self.y.write_work_bytes(out);
        self.hp.write_work_bytes(out);
    }
}

BusyWork::new(Intensity::Low)
    .feed(&game_state)
    .run();
```

### Inter-Task Data Chaining

Every `run()` call maintains a **256-byte scratch buffer** that flows through all tasks sequentially. Each task reads from the scratch buffer (blending it into its computation) and writes its results back. Task N's SHA-256 output becomes part of task N+1's sort input, which feeds task N+2's compression.

This creates a **data-dependency chain** across the entire execution — an analyzer cannot isolate individual tasks as independent noise blocks, because each task's behavior genuinely depends on what came before it.

### Loop example

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

## Categories — 84 Tasks

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

### COM (8 tasks)
WMI queries via COM automation: Win32_Process (process enumeration), Win32_OperatingSystem (OS info), Win32_ComputerSystem (computer/domain info), Win32_NetworkAdapterConfiguration (network adapters), Win32_LogicalDisk (disk info), Win32_Service (services), Win32_BIOS (BIOS info), Win32_Processor (CPU info). All **read-only** WQL SELECT queries.

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
| `cat-com`        | windows            | COM/WMI queries              |

## Anti-Detection Design

Every design decision optimizes for evasion:

| Technique | What it defeats |
|-----------|----------------|
| **Random task selection per call** | Behavioral sequence matching — no two calls produce the same API call order |
| **±30% jitter on all parameters** | Timing heuristics — iteration counts, buffer sizes, call depths all vary |
| **Data flow integration via `.feed()`** | Data-flow analysis — busywork operates on the same variables as surrounding code, preventing isolation as "noise blocks" |
| **Inter-task scratch buffer chaining** | Block isolation analysis — each task's output feeds the next task's input, creating a connected data-dependency graph across the entire run |
| **Task registry rebuilt per call** | Static analysis — no stable global function pointer table to fingerprint |
| **Function pointers, not trait objects** | Vtable signature detection — each task is a direct call to a unique address |
| **`black_box` on all results** | Dead-code elimination — compiler can't optimize away the work |
| **No time objects in binary** | String/import scanning — `Duration`, `Instant`, `SystemTime` are absent |
| **Real syscalls across categories** | Syscall sequence analysis — mixes NtReadFile, NtQueryKey, NtDeviceIoControl, etc. |
| **Read-only filesystem/registry ops** | Behavioral red flags — no writes, no creates, no deletes |
| **Legitimate API call targets** | API monitoring — calls the same APIs that normal applications use |
| **COM/WMI queries** | Process behavioral profiling — WMI queries are one of the most common Windows API patterns in legitimate software |

### What it looks like under analysis

A process calling `busywork(Medium)` in a loop produces traces like:

```
Call 1: RegOpenKeyEx → RegEnumKeyEx × 3 → RegCloseKey → SHA256 × 200 → ReadFile(hosts)
Call 2: EnumWindows → GetWindowText × 15 → GlobalMemoryStatusEx → sort(50KB)
Call 3: connect(httpbin.org:80) → send(GET) → recv → BCryptGenRandom × 8
Call 4: FindFirstFile(*.dll) × 40 → GetVolumeInformation → memcpy(16KB) × 300
Call 5: CoCreateInstance(WbemLocator) → ExecQuery(Win32_Process) → enumerate × 50
```

No repeating pattern. Each call exercises different subsystems, different buffer sizes, different iteration counts. To an EDR, it looks like a normal application doing normal things.

With `.feed()`, the data flowing through these operations comes from your actual program variables — hash chains process your session tokens, sorts operate on your counters, crypto operations encrypt your payloads. A taint-tracking analyzer following your variables will see them consumed by what appears to be legitimate processing. The scratch buffer chains these operations together so even cross-task data flow looks organic.

## Testing

185 tests across 5 test suites, verified with full untruncated output:

| Suite | Tests | What it covers |
|-------|-------|----------------|
| `workdata::tests` | 52 | Every `FeedWork` type, `blend_into` XOR correctness, `blend_seed`/`derive_usize` determinism, clone independence, empty/zero edge cases |
| `tasks::tests` | 24 | ScratchBuffer mechanics (seed, absorb, chain accumulation). Every registered task function called directly with 14 work-data shapes × parameter extremes (zero, min, large) — 1,176 task invocations. Registry completeness (84 tasks across 8 categories, no duplicates) |
| `tasks::compute::tests` | 19 | Mirror tests proving work data changes computation: hash inputs differ, fibonacci starting values shift, sieve limits get bias, cipher keys incorporate fed data, distinct inputs produce distinct seeds |
| `tasks::memory::tests` | 9 | Mirror tests proving work data flows into memory tasks: buffers modified, patterns XOR'd, ring buffers seeded, reversibility verified |
| — | — | — |
| `tests/smoke.rs` | 10 | Basic API smoke tests, category filtering (including COM), jitter toggle |
| `tests/bench.rs` | 4 | Intensity/category benchmarks, jitter variance measurement |
| `tests/feed.rs` | 64 | Data isolation (15 types verified untouched after run), per-category with feed (all 8 categories), intensity levels, edge cases (NaN, Unicode, 64KB), builder combinations, type coverage |
| `tests/robustness.rs` | 23 | Stress (500 rapid calls, 10k small feeds, 1MB feed), custom `FeedWork` impl, regression patterns (empty→real, alternating, max values) |

```
cargo test --lib -- --test-threads=1              # 85 unit tests
cargo test --test feed -- --test-threads=1        # 64 integration tests
cargo test --test robustness -- --test-threads=1  # 23 stress/regression tests
cargo test --test smoke -- --test-threads=1       # 10 smoke tests
cargo test --test bench -- --test-threads=1       # 4 benchmark tests
```

## Stats

```
84 tasks across 8 categories
All 84 tasks actively consume fed work data and participate in scratch buffer chaining
185 tests across unit, integration, stress, mirror, and benchmark suites
0 time objects in the library binary
```
