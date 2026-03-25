use crate::core::memory_stats::MemoryStats;
use crate::render::format::fmt_short;

const BAR_WIDTH: usize = 24;

enum BarShade {
    Dark,
    Mid,
    Light,
}

/// Pretty, multi-line output with monochrome bars for memory and swap sections.
pub fn render(s: &MemoryStats) -> String {
    let mem_used_pct = s.mem_used_percent();
    let mem_cache_pct = s.mem_cache_percent();
    let mem_avail_pct = s.mem_available_percent();
    let swap_pct = s.swap_used_percent();
    let swap_free_pct = s.swap_free_percent();

    let used_val = format!("{} / {}", fmt_short(s.mem_used()), fmt_short(s.mem_total));
    let cache_val = fmt_short(s.mem_cache_effective());
    let avail_val = fmt_short(s.mem_available);
    let swap_used_val = format!("{} / {}", fmt_short(s.swap_used()), fmt_short(s.swap_total));
    let swap_free_val = format!("{} / {}", fmt_short(s.swap_free), fmt_short(s.swap_total));

    let value_width = max_width(&[
        &used_val,
        &cache_val,
        &avail_val,
        &swap_used_val,
        &swap_free_val,
    ]);

    let fmt_line = |label: &str, val: &str, pct: f64, shade: BarShade| {
        format!(
            "  {:<6} {:<value_width$} {:>3.0}%  {}",
            label,
            val,
            pct,
            bar(pct, shade),
            value_width = value_width
        )
    };

    format!(
        "Memory\n{}\n{}\n{}\n\nSwap\n{}\n{}",
        fmt_line("Used:", &used_val, mem_used_pct, BarShade::Dark),
        fmt_line("Cache:", &cache_val, mem_cache_pct, BarShade::Mid),
        fmt_line("Avail:", &avail_val, mem_avail_pct, BarShade::Light),
        fmt_line("Used:", &swap_used_val, swap_pct, BarShade::Dark),
        fmt_line("Free:", &swap_free_val, swap_free_pct, BarShade::Light),
    )
}

/// Create a monochrome progress bar.
fn bar(percent: f64, shade: BarShade) -> String {
    let filled = ((percent / 100.0) * BAR_WIDTH as f64).round() as usize;
    let empty = BAR_WIDTH.saturating_sub(filled);

    let filled_char = match shade {
        BarShade::Dark => '█',
        BarShade::Mid => '▓',
        BarShade::Light => '▒',
    };

    format!(
        "{}{}",
        filled_char.to_string().repeat(filled),
        "░".repeat(empty)
    )
}

/// Calculate the maximum length of the provided string slices.
fn max_width(values: &[&str]) -> usize {
    values.iter().map(|s| s.len()).max().unwrap_or(0)
}
