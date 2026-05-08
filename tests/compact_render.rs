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
    assert_eq!(
        rendered,
        "Mem used 4GiB / 10GiB (40%) | Swap used 1GiB / 2GiB (50%)"
    );
}

#[test]
fn compact_render_handles_small_and_zero_values() {
    let stats = MemoryStats::new(999, 0, 0, 0, 0, 0, 0);

    let rendered = CompactView.render(&stats);
    assert_eq!(
        rendered,
        "Mem used 999B / 999B (100%) | Swap used 0B / 0B (0%)"
    );
}

#[test]
fn compact_render_formats_kibibyte_boundary_as_k() {
    let stats = MemoryStats::new(1024, 0, 0, 0, 0, 0, 0);

    let rendered = CompactView.render(&stats);
    assert!(rendered.starts_with("Mem used 1KiB / 1KiB"));
}
