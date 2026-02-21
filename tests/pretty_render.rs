use bfree::core::memory_stats::MemoryStats;
use bfree::render::pretty::render;

fn metric_lines(output: &str) -> Vec<&str> {
    output
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            t.starts_with("Used:")
                || t.starts_with("Cache:")
                || t.starts_with("Avail:")
                || t.starts_with("Free:")
        })
        .collect()
}

#[test]
fn pretty_render_contains_expected_sections() {
    let gib = 1024_u64.pow(3);
    let stats = MemoryStats::new(
        10 * gib,
        6 * gib,
        2 * gib,
        gib,
        512 * 1024 * 1024,
        2 * gib,
        gib,
    );

    let out = render(&stats);

    assert!(out.contains("Memory"));
    assert!(out.contains("Swap"));
    assert!(out.contains("Cache:"));
    assert!(out.contains("Avail:"));
    assert!(out.contains("Free:"));
}

#[test]
fn pretty_render_aligns_percent_column_across_rows() {
    let gib = 1024_u64.pow(3);
    let stats = MemoryStats::new(
        128 * gib,
        7 * gib,
        23 * gib,
        3 * gib,
        gib,
        512 * gib,
        511 * gib,
    );

    let out = render(&stats);
    let lines = metric_lines(&out);
    assert_eq!(lines.len(), 5, "expected 5 metric lines, got {lines:?}");

    let pct_col = lines[0].find('%').expect("missing % in first line");
    for line in &lines[1..] {
        let col = line.find('%').expect("missing % in line");
        assert_eq!(col, pct_col, "percent column misaligned: {line}");
    }
}

#[test]
fn pretty_render_aligns_bar_start_across_rows() {
    let gib = 1024_u64.pow(3);
    let stats = MemoryStats::new(64 * gib, 12 * gib, 3 * gib, gib, 0, 4 * gib, 3 * gib);

    let out = render(&stats);
    let lines = metric_lines(&out);

    let bar_col = lines[0]
        .find("\u{1b}[")
        .expect("missing ANSI color sequence");
    for line in &lines[1..] {
        let col = line.find("\u{1b}[").expect("missing ANSI color sequence");
        assert_eq!(col, bar_col, "bar start misaligned: {line}");
    }
}
