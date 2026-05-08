use bfree::core::memory_stats::MemoryStats;
use bfree::render::Renderer;
use bfree::render::views::ExtendedView;

#[test]
fn extended_render_matches_expected_layout_and_values() {
    let gib = 1024_u64.pow(3);
    let stats = MemoryStats::new(
        10 * gib,
        6 * gib,
        2 * gib,
        1 * gib,
        512 * 1024 * 1024,
        2 * gib,
        gib,
    );

    let out = ExtendedView.render(&stats);

    let expected = "memory  10GiB\n  used:        40%  4GiB\n  cache:       25%  2.5GiB\n  available:   60%  6GiB\n\nswap  2GiB\n  used:   50%  1GiB\n  free:   50%  1GiB\n\ncache breakdown\n  cached:        2GiB\n  sreclaimable:  1GiB\n  shmem:         512MiB";

    assert_eq!(out, expected);
}

#[test]
fn extended_render_handles_small_and_zero_values() {
    let stats = MemoryStats::new(999, 0, 0, 0, 0, 0, 0);

    let out = ExtendedView.render(&stats);

    assert!(out.starts_with("memory  999B"));
    assert!(out.contains("used:       100%  999B"));
    assert!(out.contains("cache:        0%  0B"));
    assert!(out.contains("available:    0%  0B"));
    assert!(out.contains("swap  0B"));
}

#[test]
fn extended_render_formats_kibibyte_boundary_as_k() {
    let stats = MemoryStats::new(1024, 0, 0, 0, 0, 0, 0);

    let out = ExtendedView.render(&stats);

    assert!(out.starts_with("memory  1KiB"));
    assert!(out.contains("used:       100%  1KiB"));
}
