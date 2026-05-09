use crate::core::memory_stats::MemoryStats;
use crate::render::Renderer;
use crate::render::format::fmt_short;

const BAR_WIDTH: usize = 24;

fn bar(percent: f64, filled_char: char) -> String {
    let filled = ((percent / 100.0) * BAR_WIDTH as f64).round() as usize;
    let empty = BAR_WIDTH.saturating_sub(filled);
    format!(
        "{}{}",
        filled_char.to_string().repeat(filled),
        "░".repeat(empty)
    )
}

fn max_width(values: &[&str]) -> usize {
    values.iter().map(|s| s.len()).max().unwrap_or(0)
}

pub struct PrettyView;

impl Renderer for PrettyView {
    fn render(&self, stats: &MemoryStats) -> String {
        let mem_used_pct = stats.mem_used_percent();
        let mem_cache_pct = stats.mem_cache_percent();
        let mem_avail_pct = stats.mem_available_percent();
        let swap_pct = stats.swap_used_percent();
        let swap_free_pct = stats.swap_free_percent();

        let used_val = format!(
            "{} / {}",
            fmt_short(stats.mem_used()),
            fmt_short(stats.mem_total)
        );
        let cache_val = fmt_short(stats.mem_cache_effective());
        let avail_val = fmt_short(stats.mem_available);
        let swap_used_val = format!(
            "{} / {}",
            fmt_short(stats.swap_used()),
            fmt_short(stats.swap_total)
        );
        let swap_free_val = format!(
            "{} / {}",
            fmt_short(stats.swap_free),
            fmt_short(stats.swap_total)
        );

        let label_width = max_width(&["used:", "cache:", "available:", "free:"]);
        let value_width = max_width(&[
            &used_val,
            &cache_val,
            &avail_val,
            &swap_used_val,
            &swap_free_val,
        ]);

        let fmt_line = |label: &str, val: &str, pct: f64, filled_char: char| {
            format!(
                "  {:<label_width$} {:>3.0}%  {:<value_width$}  {}",
                label,
                pct,
                val,
                bar(pct, filled_char),
                label_width = label_width,
                value_width = value_width
            )
        };

        format!(
            "memory\n{}\n{}\n{}\n\nswap\n{}\n{}",
            fmt_line("used:", &used_val, mem_used_pct, '█'),
            fmt_line("cache:", &cache_val, mem_cache_pct, '▓'),
            fmt_line("available:", &avail_val, mem_avail_pct, '▒'),
            fmt_line("used:", &swap_used_val, swap_pct, '█'),
            fmt_line("free:", &swap_free_val, swap_free_pct, '▒'),
        )
    }
}
