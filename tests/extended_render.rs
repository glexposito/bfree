use bfree::core::memory_stats::MemoryStats;
use bfree::render::Renderer;
use bfree::render::extended::ExtendedRenderer;

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

    let out = ExtendedRenderer::new().render(&stats);

    let expected = "Memory\n  Total:        10GiB\n  Used:         4GiB (40%)\n  Cache:        2.5GiB (25%)\n  Available:    6GiB (60%)\n\nSwap\n  Total:        2GiB\n  Used:         1GiB (50%)\n  Free:         1GiB (50%)\n\nCache Breakdown\n  Cached:        2GiB\n  SReclaimable:  1GiB\n  Shmem:         512MiB";

    assert_eq!(out, expected);
}

#[test]
fn extended_render_handles_small_and_zero_values() {
    let stats = MemoryStats::new(999, 0, 0, 0, 0, 0, 0);

    let out = ExtendedRenderer::new().render(&stats);

    assert!(out.contains("Total:        999B"));
    assert!(out.contains("Used:         999B (100%)"));
    assert!(out.contains("Cache:        0B (0%)"));
    assert!(out.contains("Available:    0B (0%)"));
    assert!(
        out.contains("Swap\n  Total:        0B\n  Used:         0B (0%)\n  Free:         0B (0%)")
    );
}

#[test]
fn extended_render_formats_kibibyte_boundary_as_k() {
    let stats = MemoryStats::new(1024, 0, 0, 0, 0, 0, 0);

    let out = ExtendedRenderer::new().render(&stats);

    assert!(out.contains("Total:        1KiB"));
    assert!(out.contains("Used:         1KiB (100%)"));
}
