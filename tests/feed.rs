use busywork::{BusyWork, Categories, Intensity};

// ════════════════════════════════════════════════════════════════════════════
// DATA ISOLATION: originals must never be modified after feed + run
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn original_u8_untouched() {
    let val: u8 = 0xFF;
    BusyWork::new(Intensity::Low).allow(Categories::COMPUTE).feed(&val).run();
    assert_eq!(val, 0xFF);
}

#[test]
fn original_u16_untouched() {
    let val: u16 = 12345;
    BusyWork::new(Intensity::Low).allow(Categories::COMPUTE).feed(&val).run();
    assert_eq!(val, 12345);
}

#[test]
fn original_u32_untouched() {
    let val: u32 = 42;
    BusyWork::new(Intensity::Low).allow(Categories::COMPUTE).feed(&val).run();
    assert_eq!(val, 42);
}

#[test]
fn original_u64_untouched() {
    let val: u64 = 0xDEAD_BEEF_CAFE_BABE;
    BusyWork::new(Intensity::Low).allow(Categories::COMPUTE).feed(&val).run();
    assert_eq!(val, 0xDEAD_BEEF_CAFE_BABE);
}

#[test]
fn original_u128_untouched() {
    let val: u128 = u128::MAX;
    BusyWork::new(Intensity::Low).allow(Categories::COMPUTE).feed(&val).run();
    assert_eq!(val, u128::MAX);
}

#[test]
fn original_i32_untouched() {
    let val: i32 = -42;
    BusyWork::new(Intensity::Low).allow(Categories::COMPUTE).feed(&val).run();
    assert_eq!(val, -42);
}

#[test]
fn original_i64_untouched() {
    let val: i64 = i64::MIN;
    BusyWork::new(Intensity::Low).allow(Categories::COMPUTE).feed(&val).run();
    assert_eq!(val, i64::MIN);
}

#[test]
fn original_f32_untouched() {
    let val: f32 = std::f32::consts::PI;
    BusyWork::new(Intensity::Low).allow(Categories::COMPUTE).feed(&val).run();
    assert_eq!(val, std::f32::consts::PI);
}

#[test]
fn original_f64_untouched() {
    let val: f64 = std::f64::consts::E;
    BusyWork::new(Intensity::Low).allow(Categories::COMPUTE).feed(&val).run();
    assert_eq!(val, std::f64::consts::E);
}

#[test]
fn original_bool_untouched() {
    let val = true;
    BusyWork::new(Intensity::Low).allow(Categories::COMPUTE).feed(&val).run();
    assert!(val);
}

#[test]
fn original_string_untouched() {
    let val = String::from("sensitive_data_do_not_modify");
    BusyWork::new(Intensity::Low).allow(Categories::COMPUTE).feed(&val).run();
    assert_eq!(val, "sensitive_data_do_not_modify");
}

#[test]
fn original_vec_untouched() {
    let val = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    BusyWork::new(Intensity::Low).allow(Categories::MEMORY).feed(&val).run();
    assert_eq!(val, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn original_array_untouched() {
    let val: [u8; 8] = [0xCA, 0xFE, 0xBA, 0xBE, 0xDE, 0xAD, 0xBE, 0xEF];
    BusyWork::new(Intensity::Low).allow(Categories::COMPUTE).feed(&val).run();
    assert_eq!(val, [0xCA, 0xFE, 0xBA, 0xBE, 0xDE, 0xAD, 0xBE, 0xEF]);
}

#[test]
fn original_multiple_vars_untouched() {
    let a: u32 = 42;
    let b = String::from("hello");
    let c = vec![0xAAu8, 0xBB];
    let d: f64 = 3.14;
    let e = true;

    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE | Categories::MEMORY)
        .feed(&a)
        .feed(&b)
        .feed(&c)
        .feed(&d)
        .feed(&e)
        .run();

    assert_eq!(a, 42);
    assert_eq!(b, "hello");
    assert_eq!(c, vec![0xAA, 0xBB]);
    assert_eq!(d, 3.14);
    assert!(e);
}

// ════════════════════════════════════════════════════════════════════════════
// PER-CATEGORY SMOKE WITH FEED DATA
// ════════════════════════════════════════════════════════════════════════════

fn sample_feed() -> BusyWork {
    BusyWork::new(Intensity::Low)
        .feed(&42u32)
        .feed(&0xDEADBEEFu64)
        .feed("test_context_data")
        .feed(&vec![0xAAu8, 0xBB, 0xCC, 0xDD])
        .feed(&3.14f64)
        .feed(&true)
}

#[test]
fn feed_compute_no_panic() {
    sample_feed().allow(Categories::COMPUTE).run();
}

#[test]
fn feed_memory_no_panic() {
    sample_feed().allow(Categories::MEMORY).run();
}

#[test]
fn feed_filesystem_no_panic() {
    sample_feed().allow(Categories::FILESYSTEM).run();
}

#[test]
fn feed_registry_no_panic() {
    sample_feed().allow(Categories::REGISTRY).run();
}

#[test]
fn feed_winapi_no_panic() {
    sample_feed().allow(Categories::WINAPI).run();
}

#[test]
fn feed_crypto_no_panic() {
    sample_feed().allow(Categories::CRYPTO).run();
}

#[test]
fn feed_com_no_panic() {
    sample_feed().allow(Categories::COM).run();
}

#[test]
fn feed_all_categories_no_panic() {
    sample_feed().deny(Categories::NETWORK).run();
}

// ════════════════════════════════════════════════════════════════════════════
// INTENSITY LEVELS WITH FEED DATA
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn feed_low_compute() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&42u32)
        .feed("low_test")
        .run();
}

#[test]
fn feed_low_memory() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::MEMORY)
        .feed(&42u32)
        .feed(&vec![1u8, 2, 3, 4])
        .run();
}

#[test]
fn feed_medium_compute() {
    BusyWork::new(Intensity::Medium)
        .allow(Categories::COMPUTE)
        .feed(&42u32)
        .feed("medium_test")
        .run();
}

#[test]
fn feed_medium_memory() {
    BusyWork::new(Intensity::Medium)
        .allow(Categories::MEMORY)
        .feed(&0xCAFEBABEu32)
        .run();
}

#[test]
fn feed_high_compute() {
    BusyWork::new(Intensity::High)
        .allow(Categories::COMPUTE)
        .feed(&42u32)
        .run();
}

#[test]
fn feed_high_memory() {
    BusyWork::new(Intensity::High)
        .allow(Categories::MEMORY)
        .feed(&42u32)
        .run();
}

// ════════════════════════════════════════════════════════════════════════════
// EDGE CASES
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn feed_empty_string() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed("")
        .run();
}

#[test]
fn feed_empty_vec() {
    let empty: Vec<u8> = vec![];
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&empty)
        .run();
}

#[test]
fn feed_single_byte() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&0xFFu8)
        .run();
}

#[test]
fn feed_all_zeros() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE | Categories::MEMORY)
        .feed(&0u8)
        .feed(&0u16)
        .feed(&0u32)
        .feed(&0u64)
        .feed(&0.0f32)
        .feed(&0.0f64)
        .feed(&false)
        .run();
}

#[test]
fn feed_max_values() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&u8::MAX)
        .feed(&u16::MAX)
        .feed(&u32::MAX)
        .feed(&u64::MAX)
        .feed(&i8::MIN)
        .feed(&i16::MIN)
        .feed(&i32::MIN)
        .feed(&i64::MIN)
        .feed(&f64::MAX)
        .feed(&f64::MIN)
        .run();
}

#[test]
fn feed_special_floats() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&f32::NAN)
        .feed(&f64::INFINITY)
        .feed(&f64::NEG_INFINITY)
        .feed(&f64::MIN_POSITIVE)
        .feed(&f64::EPSILON)
        .run();
}

#[test]
fn feed_large_data_64kb() {
    let big = vec![0xABu8; 64 * 1024];
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE | Categories::MEMORY)
        .feed(&big)
        .run();
}

#[test]
fn feed_same_value_repeated() {
    let val = 42u32;
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&val)
        .feed(&val)
        .feed(&val)
        .run();
}

#[test]
fn feed_256_u8_values() {
    let mut builder = BusyWork::new(Intensity::Low).allow(Categories::COMPUTE);
    for i in 0u8..=255 {
        builder = builder.feed(&i);
    }
    builder.run();
}

#[test]
fn feed_100_u64_values() {
    let mut builder = BusyWork::new(Intensity::Low).allow(Categories::COMPUTE);
    for i in 0u64..100 {
        builder = builder.feed(&i);
    }
    builder.run();
}

#[test]
fn feed_long_string() {
    let long = "A".repeat(10_000);
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&long)
        .run();
}

#[test]
fn feed_unicode_string() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed("Ünïcödé Strïñg 日本語 中文 🎉")
        .run();
}

// ════════════════════════════════════════════════════════════════════════════
// BUILDER COMBINATIONS
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn feed_with_deny() {
    BusyWork::new(Intensity::Medium)
        .deny(Categories::NETWORK)
        .feed(&42u32)
        .feed("context")
        .run();
}

#[test]
fn feed_with_allow_and_deny() {
    BusyWork::new(Intensity::Medium)
        .allow(Categories::COMPUTE | Categories::MEMORY | Categories::NETWORK)
        .deny(Categories::NETWORK)
        .feed(&42u32)
        .run();
}

#[test]
fn feed_with_jitter_off() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .jitter(false)
        .feed(&42u32)
        .feed("deterministic")
        .run();
}

#[test]
fn feed_with_jitter_on() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .jitter(true)
        .feed(&42u32)
        .run();
}

#[test]
fn feed_with_empty_categories() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::empty())
        .feed(&42u32)
        .feed("should be safe even with no categories")
        .run();
}

#[test]
fn feed_before_allow() {
    BusyWork::new(Intensity::Low)
        .feed(&42u32)
        .allow(Categories::COMPUTE)
        .run();
}

#[test]
fn feed_after_allow() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&42u32)
        .run();
}

#[test]
fn feed_interleaved_with_builder_options() {
    BusyWork::new(Intensity::Low)
        .feed(&1u32)
        .allow(Categories::COMPUTE | Categories::MEMORY)
        .feed(&2u64)
        .deny(Categories::NETWORK)
        .feed("interleaved")
        .jitter(false)
        .feed(&3.14f64)
        .run();
}

#[test]
fn feed_only_deny_no_allow() {
    BusyWork::new(Intensity::Low)
        .deny(Categories::NETWORK | Categories::REGISTRY)
        .feed(&42u32)
        .run();
}

// ════════════════════════════════════════════════════════════════════════════
// TYPE COVERAGE: every FeedWork implementation path
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn feed_all_unsigned_integers() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&1u8)
        .feed(&2u16)
        .feed(&3u32)
        .feed(&4u64)
        .feed(&5u128)
        .feed(&6usize)
        .run();
}

#[test]
fn feed_all_signed_integers() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&(-1i8))
        .feed(&(-2i16))
        .feed(&(-3i32))
        .feed(&(-4i64))
        .feed(&(-5i128))
        .feed(&(-6isize))
        .run();
}

#[test]
fn feed_all_float_types() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&1.0f32)
        .feed(&2.0f64)
        .run();
}

#[test]
fn feed_str_literal_directly() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed("literal")
        .run();
}

#[test]
fn feed_string_by_ref() {
    let s = String::from("owned");
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&s)
        .run();
}

#[test]
fn feed_byte_slice_from_vec() {
    let v = vec![1u8, 2, 3];
    let slice: &[u8] = &v;
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(slice)
        .run();
}

#[test]
fn feed_fixed_array() {
    let arr: [u8; 16] = [0; 16];
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&arr)
        .run();
}

#[test]
fn feed_boxed_value() {
    let boxed: Box<u32> = Box::new(42);
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&boxed)
        .run();
}

#[test]
fn feed_boxed_bytes() {
    let boxed: Box<[u8]> = vec![1u8, 2, 3].into_boxed_slice();
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(&boxed)
        .run();
}

#[test]
fn feed_double_reference() {
    let val = 42u32;
    let ref1 = &val;
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .feed(ref1)
        .run();
}

// ════════════════════════════════════════════════════════════════════════════
// WITHOUT FEED: existing behavior unbroken
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn no_feed_compute() {
    BusyWork::new(Intensity::Low).allow(Categories::COMPUTE).run();
}

#[test]
fn no_feed_memory() {
    BusyWork::new(Intensity::Low).allow(Categories::MEMORY).run();
}

#[test]
fn no_feed_all_categories() {
    BusyWork::new(Intensity::Low).deny(Categories::NETWORK).run();
}

#[test]
fn no_feed_with_jitter_off() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE)
        .jitter(false)
        .run();
}

// ════════════════════════════════════════════════════════════════════════════
// NETWORK (separate, slower due to timeouts)
// ════════════════════════════════════════════════════════════════════════════

#[test]
#[ignore]
fn feed_network_no_panic() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::NETWORK)
        .feed(&80u16)
        .feed("network_context")
        .run();
}
