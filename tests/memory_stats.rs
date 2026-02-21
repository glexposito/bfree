use bfree::core::memory_stats::MemoryStats;

fn assert_close(actual: f64, expected: f64, epsilon: f64) {
    let diff = (actual - expected).abs();
    assert!(
        diff <= epsilon,
        "expected {expected}, got {actual}, diff {diff} > {epsilon}"
    );
}

#[test]
fn new_sets_all_fields() {
    let stats = MemoryStats::new(1_000, 400, 100, 50, 20, 800, 300);

    assert_eq!(stats.mem_total, 1_000);
    assert_eq!(stats.mem_available, 400);
    assert_eq!(stats.mem_cached, 100);
    assert_eq!(stats.mem_sreclaimable, 50);
    assert_eq!(stats.mem_shmem, 20);
    assert_eq!(stats.swap_total, 800);
    assert_eq!(stats.swap_free, 300);
}

#[test]
fn computes_memory_and_swap_metrics() {
    let stats = MemoryStats::new(1_000, 400, 100, 50, 20, 800, 300);

    assert_eq!(stats.mem_used(), 600);
    assert_eq!(stats.mem_cache_effective(), 130);
    assert_eq!(stats.swap_used(), 500);

    assert_close(stats.mem_used_percent(), 60.0, 1e-10);
    assert_close(stats.mem_cache_percent(), 13.0, 1e-10);
    assert_close(stats.mem_available_percent(), 40.0, 1e-10);
    assert_close(stats.swap_used_percent(), 62.5, 1e-10);
}

#[test]
fn clamps_subtraction_underflow_to_zero() {
    let stats = MemoryStats::new(1_000, 1_500, 20, 10, 50, 100, 200);

    assert_eq!(stats.mem_used(), 0);
    assert_eq!(stats.mem_cache_effective(), 0);
    assert_eq!(stats.swap_used(), 0);
    assert_close(stats.mem_used_percent(), 0.0, 1e-10);
    assert_close(stats.mem_cache_percent(), 0.0, 1e-10);
    assert_close(stats.swap_used_percent(), 0.0, 1e-10);
}

#[test]
fn returns_zero_percent_when_totals_are_zero() {
    let stats = MemoryStats::new(0, 0, 0, 0, 0, 0, 0);

    assert_close(stats.mem_used_percent(), 0.0, 1e-10);
    assert_close(stats.mem_cache_percent(), 0.0, 1e-10);
    assert_close(stats.mem_available_percent(), 0.0, 1e-10);
    assert_close(stats.swap_used_percent(), 0.0, 1e-10);
}
