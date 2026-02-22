use bfree::core::memory_stats::MemoryStats;
use bfree::render::compact::render;

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

    let rendered = render(&stats);
    assert_eq!(
        rendered,
        "Mem 10.0G used 4.0G (40%) avail 6.0G (60%) | Swap 2.0G used 1.0G (50%)"
    );
}

#[test]
fn compact_render_handles_small_and_zero_values() {
    let stats = MemoryStats::new(999, 0, 0, 0, 0, 0, 0);

    let rendered = render(&stats);
    assert_eq!(
        rendered,
        "Mem 999B used 999B (100%) avail 0B (0%) | Swap 0B used 0B (0%)"
    );
}

#[test]
fn compact_render_formats_kibibyte_boundary_as_k() {
    let stats = MemoryStats::new(1024, 0, 0, 0, 0, 0, 0);

    let rendered = render(&stats);
    assert!(rendered.starts_with("Mem 1K used 1K"));
}
