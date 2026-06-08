use busywork::{BusyWork, Categories, FeedWork, Intensity, WorkData};

// ════════════════════════════════════════════════════════════════════════════
// STRESS: rapid sequential calls with varying data
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn stress_500_calls_varying_u32() {
    for i in 0u32..500 {
        BusyWork::new(Intensity::Low)
            .allow(Categories::COMPUTE)
            .jitter(false)
            .feed(&i)
            .run();
    }
}

#[test]
fn stress_200_calls_varying_bytes() {
    for i in 0u16..200 {
        let data = i.to_ne_bytes();
        BusyWork::new(Intensity::Low)
            .allow(Categories::COMPUTE | Categories::MEMORY)
            .feed(&data)
            .run();
    }
}

#[test]
fn stress_100_calls_same_data() {
    let data = vec![0xABu8; 256];
    for _ in 0..100 {
        BusyWork::new(Intensity::Low)
            .allow(Categories::COMPUTE)
            .feed(&data)
            .run();
    }
}

#[test]
fn stress_50_calls_growing_data() {
    for size in 0..50 {
        let data = vec![0xCDu8; size * 100];
        BusyWork::new(Intensity::Low)
            .allow(Categories::COMPUTE)
            .feed(&data)
            .run();
    }
}

// ════════════════════════════════════════════════════════════════════════════
// EXTREME DATA SIZES
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn extreme_1mb_feed() {
    let big = vec![0xEFu8; 1024 * 1024];
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE | Categories::MEMORY)
        .feed(&big)
        .run();
}

#[test]
fn extreme_10000_small_feeds() {
    let mut builder = BusyWork::new(Intensity::Low).allow(Categories::COMPUTE);
    for i in 0u16..10_000 {
        builder = builder.feed(&i);
    }
    builder.run();
}

#[test]
fn extreme_feed_then_empty_categories() {
    let big = vec![0xFFu8; 64 * 1024];
    BusyWork::new(Intensity::Ultra)
        .allow(Categories::empty())
        .feed(&big)
        .run();
}

// ════════════════════════════════════════════════════════════════════════════
// WORKDATA DIRECT API
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn workdata_feed_many_types_no_panic() {
    let mut wd = WorkData::new();
    wd.feed(&0u8);
    wd.feed(&0u16);
    wd.feed(&0u32);
    wd.feed(&0u64);
    wd.feed(&0u128);
    wd.feed(&0usize);
    wd.feed(&0i8);
    wd.feed(&0i16);
    wd.feed(&0i32);
    wd.feed(&0i64);
    wd.feed(&0i128);
    wd.feed(&0isize);
    wd.feed(&0.0f32);
    wd.feed(&0.0f64);
    wd.feed(&false);
    wd.feed(&true);
    wd.feed("");
    wd.feed("hello");
    wd.feed(&String::from("world"));
    let v = vec![1u8, 2, 3];
    wd.feed(&v);
    let s: &[u8] = &[4, 5, 6];
    wd.feed(s);
    wd.feed(&[7u8, 8, 9]);
    let boxed: Box<u32> = Box::new(42);
    wd.feed(&boxed);
}

#[test]
fn workdata_clone_divergence() {
    let mut a = WorkData::new();
    a.feed(&42u32);
    let mut b = a.clone();
    b.feed(&99u32);
    // After divergence, using both should be safe
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&42u32)
        .run();
}

#[test]
fn workdata_reuse_across_runs() {
    let mut wd = WorkData::new();
    wd.feed(&42u32);
    wd.feed("persistent_context");

    for _ in 0..10 {
        let mut builder = BusyWork::new(Intensity::Low).allow(Categories::COMPUTE);
        // Re-feed same data (simulating reuse pattern)
        builder = builder.feed(&42u32);
        builder = builder.feed("persistent_context");
        builder.run();
    }
}

// ════════════════════════════════════════════════════════════════════════════
// CUSTOM TYPE: user implements FeedWork for their own struct
// ════════════════════════════════════════════════════════════════════════════

struct GameState {
    player_x: f32,
    player_y: f32,
    health: u32,
    score: u64,
}

impl FeedWork for GameState {
    fn write_work_bytes(&self, out: &mut Vec<u8>) {
        self.player_x.write_work_bytes(out);
        self.player_y.write_work_bytes(out);
        self.health.write_work_bytes(out);
        self.score.write_work_bytes(out);
    }
}

#[test]
fn custom_type_feed() {
    let state = GameState {
        player_x: 100.5,
        player_y: -200.3,
        health: 75,
        score: 12345678,
    };
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&state)
        .run();
}

#[test]
fn custom_type_original_untouched() {
    let state = GameState {
        player_x: 100.5,
        player_y: -200.3,
        health: 75,
        score: 12345678,
    };
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&state)
        .run();
    assert_eq!(state.player_x, 100.5);
    assert_eq!(state.player_y, -200.3);
    assert_eq!(state.health, 75);
    assert_eq!(state.score, 12345678);
}

// ════════════════════════════════════════════════════════════════════════════
// MIXED CATEGORIES WITH FEED
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn feed_compute_and_memory() {
    BusyWork::new(Intensity::Medium)
        .allow(Categories::COMPUTE | Categories::MEMORY)
        .feed(&42u32)
        .feed(&vec![0xAB; 128])
        .run();
}

#[test]
fn feed_compute_and_filesystem() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE | Categories::FILESYSTEM)
        .feed(&42u32)
        .run();
}

#[test]
fn feed_compute_and_crypto() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE | Categories::CRYPTO)
        .feed(&42u32)
        .feed("crypto_context")
        .run();
}

#[test]
fn feed_memory_and_crypto() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::MEMORY | Categories::CRYPTO)
        .feed(&0xCAFEBABEu32)
        .run();
}

#[test]
fn feed_all_except_network() {
    BusyWork::new(Intensity::Medium)
        .deny(Categories::NETWORK)
        .feed(&42u32)
        .feed("full_context_test")
        .feed(&vec![1u8, 2, 3, 4, 5])
        .feed(&3.14f64)
        .run();
}

// ════════════════════════════════════════════════════════════════════════════
// REGRESSION: specific patterns that could trigger bugs
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn regression_single_zero_byte_feed() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE | Categories::MEMORY)
        .feed(&0u8)
        .run();
}

#[test]
fn regression_feed_after_empty_feed() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&Vec::<u8>::new())
        .feed(&42u32)
        .run();
}

#[test]
fn regression_many_empty_feeds_then_real() {
    let mut builder = BusyWork::new(Intensity::Low).allow(Categories::COMPUTE);
    for _ in 0..100 {
        builder = builder.feed("");
    }
    builder = builder.feed(&42u32);
    builder.run();
}

#[test]
fn regression_alternating_empty_and_data() {
    let mut builder = BusyWork::new(Intensity::Low).allow(Categories::COMPUTE);
    for i in 0u32..50 {
        builder = builder.feed("");
        builder = builder.feed(&i);
        builder = builder.feed(&Vec::<u8>::new());
    }
    builder.run();
}

#[test]
fn regression_max_u64_feed() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&u64::MAX)
        .run();
}

#[test]
fn regression_all_ones_large() {
    let data = vec![0xFFu8; 8192];
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE | Categories::MEMORY)
        .feed(&data)
        .run();
}
