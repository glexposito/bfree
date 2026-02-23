#[cfg(target_os = "linux")]
use bfree::platform::linux;

#[cfg(target_os = "linux")]
#[test]
fn read_memory_stats_smoke_test() {
    let stats = linux::read_memory_stats().expect("failed to read linux memory stats");
    assert!(stats.mem_total > 0, "mem_total should be positive");
}
