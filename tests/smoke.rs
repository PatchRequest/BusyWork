use busywork::{busywork, busywork_with, BusyWork, Categories, Intensity};

#[test]
fn low_intensity_no_panic() {
    busywork(Intensity::Low);
}

#[test]
fn medium_intensity_no_panic() {
    busywork(Intensity::Medium);
}

#[test]
fn high_compute_only() {
    busywork_with(Intensity::High, Categories::COMPUTE);
}

#[test]
fn high_memory_only() {
    busywork_with(Intensity::High, Categories::MEMORY);
}

#[test]
fn high_filesystem_only() {
    busywork_with(Intensity::High, Categories::FILESYSTEM);
}

#[test]
fn builder_deny_network() {
    BusyWork::new(Intensity::Medium)
        .deny(Categories::NETWORK)
        .run();
}

#[test]
fn builder_no_jitter() {
    BusyWork::new(Intensity::Low)
        .allow(Categories::COMPUTE | Categories::MEMORY)
        .jitter(false)
        .run();
}

#[test]
fn empty_categories_no_panic() {
    busywork_with(Intensity::Ultra, Categories::empty());
}

#[test]
fn available_returns_all_compiled() {
    let avail = Categories::available();
    assert!(avail.contains(Categories::COMPUTE));
    assert!(avail.contains(Categories::MEMORY));
    assert!(avail.contains(Categories::FILESYSTEM));
    assert!(avail.contains(Categories::COM));
}

#[test]
fn high_com_only() {
    busywork_with(Intensity::High, Categories::COM);
}
