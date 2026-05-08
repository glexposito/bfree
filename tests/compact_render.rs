use bfree::core::memory_stats::MemoryStats;
use bfree::render::Renderer;
use bfree::render::views::CompactView;

#[test]
fn compact_render_renders_expected_layout_and_percentages() {
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

    let rendered = CompactView.render(&stats);
    assert_eq!(rendered, "mem    40%  4GiB / 10GiB\nswap   50%  1GiB / 2GiB");
}

#[test]
fn compact_render_handles_small_and_zero_values() {
    let stats = MemoryStats::new(999, 0, 0, 0, 0, 0, 0);

    let rendered = CompactView.render(&stats);
    assert_eq!(rendered, "mem   100%  999B / 999B\nswap    0%  0B   / 0B");
}

#[test]
fn compact_render_formats_kibibyte_boundary_as_k() {
    let stats = MemoryStats::new(1024, 0, 0, 0, 0, 0, 0);

    let rendered = CompactView.render(&stats);
    assert!(rendered.starts_with("mem   100%  1KiB / 1KiB"));
}
