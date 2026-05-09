use bfree::core::memory_stats::MemoryStats;
use bfree::render::Renderer;
use bfree::render::views::CompactView;

#[test]
fn compact_render_renders_expected_layout_and_percentages() {
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

    let rendered = CompactView.render(&stats);
    assert_eq!(rendered, "mem    40%  4GiB / 10GiB\nswap   50%  1GiB / 2GiB");
}

#[test]
fn compact_render_handles_small_and_zero_values() {
    let stats = MemoryStats {
        mem_total: 999,
        mem_available: 0,
        mem_cached: 0,
        mem_sreclaimable: 0,
        mem_shmem: 0,
        swap_total: 0,
        swap_free: 0,
    };

    let rendered = CompactView.render(&stats);
    assert_eq!(rendered, "mem   100%  999B / 999B\nswap    0%  0B   / 0B");
}

#[test]
fn compact_render_formats_kibibyte_boundary_as_k() {
    let stats = MemoryStats {
        mem_total: 1024,
        mem_available: 0,
        mem_cached: 0,
        mem_sreclaimable: 0,
        mem_shmem: 0,
        swap_total: 0,
        swap_free: 0,
    };

    let rendered = CompactView.render(&stats);
    assert!(rendered.starts_with("mem   100%  1KiB / 1KiB"));
}
