use bfree::core::memory_stats::MemoryStats;
use bfree::render::text::one_line;

#[test]
fn one_line_renders_expected_layout_and_percentages() {
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

    let rendered = one_line(&stats);
    assert_eq!(
        rendered,
        "Mem 10.0G used 4.0G (40%) cache 2.5G (25%) avail 6.0G (60%) | Swap 2.0G used 1.0G (50%)"
    );
}

#[test]
fn one_line_handles_small_and_zero_values() {
    let stats = MemoryStats::new(999, 0, 0, 0, 0, 0, 0);

    let rendered = one_line(&stats);
    assert_eq!(
        rendered,
        "Mem 999B used 999B (100%) cache 0B (0%) avail 0B (0%) | Swap 0B used 0B (0%)"
    );
}

#[test]
fn one_line_formats_kibibyte_boundary_as_k() {
    let stats = MemoryStats::new(1024, 0, 0, 0, 0, 0, 0);

    let rendered = one_line(&stats);
    assert!(rendered.starts_with("Mem 1K used 1K"));
}
