use bfree::core::memory_stats::MemoryStats;
use bfree::render::Renderer;
use bfree::render::views::ExtendedView;

#[test]
fn extended_render_matches_expected_layout_and_values() {
    let gib = 1024_u64.pow(3);
    let stats = MemoryStats {
        mem_total: 10 * gib,
        mem_available: 6 * gib,
        mem_cached: 2 * gib,
        mem_sreclaimable: 1 * gib,
        mem_shmem: 512 * 1024 * 1024,
        swap_total: 2 * gib,
        swap_free: gib,
    };

    let out = ExtendedView.render(&stats);

    let expected = "memory\n  used:        40%  4GiB / 10GiB\n  cache:       25%  2.5GiB\n  available:   60%  6GiB\n\nswap\n  used:   50%  1GiB / 2GiB\n  free:   50%  1GiB\n\ncache breakdown\n  cached:        2GiB\n  sreclaimable:  1GiB\n  shmem:         512MiB";

    assert_eq!(out, expected);
}

#[test]
fn extended_render_handles_small_and_zero_values() {
    let stats = MemoryStats {
        mem_total: 999,
        mem_available: 0,
        mem_cached: 0,
        mem_sreclaimable: 0,
        mem_shmem: 0,
        swap_total: 0,
        swap_free: 0,
    };

    let out = ExtendedView.render(&stats);

    assert!(out.starts_with("memory\n"));
    assert!(out.contains("used:       100%  999B / 999B"));
    assert!(out.contains("cache:        0%  0B"));
    assert!(out.contains("available:    0%  0B"));
    assert!(out.contains("swap\n"));
}

#[test]
fn extended_render_formats_kibibyte_boundary_as_k() {
    let stats = MemoryStats {
        mem_total: 1024,
        mem_available: 0,
        mem_cached: 0,
        mem_sreclaimable: 0,
        mem_shmem: 0,
        swap_total: 0,
        swap_free: 0,
    };

    let out = ExtendedView.render(&stats);

    assert!(out.starts_with("memory\n"));
    assert!(out.contains("used:       100%  1KiB / 1KiB"));
}
