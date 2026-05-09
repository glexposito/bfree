use bfree::core::memory_stats::MemoryStats;
use bfree::render::Renderer;
use bfree::render::views::PrettyView;

fn metric_lines(output: &str) -> Vec<&str> {
    output
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            t.starts_with("used:")
                || t.starts_with("cache:")
                || t.starts_with("avail:")
                || t.starts_with("free:")
        })
        .collect()
}

#[test]
fn pretty_render_contains_expected_sections() {
    let gib = 1024_u64.pow(3);
    let stats = MemoryStats {
        mem_total: 10 * gib,
        mem_available: 6 * gib,
        mem_cached: 2 * gib,
        mem_sreclaimable: gib,
        mem_shmem: 512 * 1024 * 1024,
        swap_total: 2 * gib,
        swap_free: gib,
    };

    let out = PrettyView.render(&stats);

    assert!(out.contains("memory"));
    assert!(out.contains("swap"));
    assert!(out.contains("cache:"));
    assert!(out.contains("avail:"));
    assert!(out.contains("free:"));
}

#[test]
fn pretty_render_aligns_percent_column_across_rows() {
    let gib = 1024_u64.pow(3);
    let stats = MemoryStats {
        mem_total: 128 * gib,
        mem_available: 7 * gib,
        mem_cached: 23 * gib,
        mem_sreclaimable: 3 * gib,
        mem_shmem: gib,
        swap_total: 512 * gib,
        swap_free: 511 * gib,
    };

    let out = PrettyView.render(&stats);
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
    let stats = MemoryStats {
        mem_total: 64 * gib,
        mem_available: 12 * gib,
        mem_cached: 3 * gib,
        mem_sreclaimable: gib,
        mem_shmem: 0,
        swap_total: 4 * gib,
        swap_free: 3 * gib,
    };

    let out = PrettyView.render(&stats);
    let lines = metric_lines(&out);

    let bar_col = lines[0]
        .find(['█', '▓', '▒', '░'])
        .expect("missing bar glyph");
    for line in &lines[1..] {
        let col = line.find(['█', '▓', '▒', '░']).expect("missing bar glyph");
        assert_eq!(col, bar_col, "bar start misaligned: {line}");
    }
}
