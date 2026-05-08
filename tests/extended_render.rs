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

    let expected = "memory\n  total:        10GiB\n  used:         4GiB (40%)\n  cache:        2.5GiB (25%)\n  available:    6GiB (60%)\n\nswap\n  total:        2GiB\n  used:         1GiB (50%)\n  free:         1GiB (50%)\n\ncache breakdown\n  cached:        2GiB\n  sreclaimable:  1GiB\n  shmem:         512MiB";

    assert_eq!(out, expected);
}

#[test]
fn extended_render_handles_small_and_zero_values() {
    let stats = MemoryStats::new(999, 0, 0, 0, 0, 0, 0);

    let out = ExtendedView.render(&stats);

    assert!(out.contains("total:        999B"));
    assert!(out.contains("used:         999B (100%)"));
    assert!(out.contains("cache:        0B (0%)"));
    assert!(out.contains("available:    0B (0%)"));
    assert!(out.contains("swap\n  total:        0B\n  used:         0B (0%)\n  free:         0B (0%)"));
}

#[test]
fn extended_render_formats_kibibyte_boundary_as_k() {
    let stats = MemoryStats::new(1024, 0, 0, 0, 0, 0, 0);

    let out = ExtendedView.render(&stats);

    assert!(out.contains("total:        1KiB"));
    assert!(out.contains("used:         1KiB (100%)"));
}
