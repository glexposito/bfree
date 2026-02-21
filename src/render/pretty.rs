use crate::core::memory_stats::MemoryStats;
use crate::render::format::fmt_short;

const BAR_WIDTH: usize = 24;

enum BarTone {
    Danger,
    Warning,
    Good,
}

pub fn render(s: &MemoryStats) -> String {
    let mem_used_pct = s.mem_used_percent();
    let mem_cache_pct = s.mem_cache_percent();
    let mem_avail_pct = s.mem_available_percent();
    let swap_pct = s.swap_used_percent();
    let swap_free_pct = if s.swap_total == 0 {
        0.0
    } else {
        (s.swap_free as f64) * 100.0 / (s.swap_total as f64)
    };
    let used_value = format!("{} / {}", fmt_short(s.mem_used()), fmt_short(s.mem_total));
    let cache_value = fmt_short(s.mem_cache_effective());
    let avail_value = fmt_short(s.mem_available);
    let swap_used_value = format!("{} / {}", fmt_short(s.swap_used()), fmt_short(s.swap_total));
    let swap_free_value = format!("{} / {}", fmt_short(s.swap_free), fmt_short(s.swap_total));
    let value_width = [
        used_value.len(),
        cache_value.len(),
        avail_value.len(),
        swap_used_value.len(),
        swap_free_value.len(),
    ]
    .into_iter()
    .max()
    .unwrap_or(0);

    let used_line = format!(
        "  {:<6} {:<value_width$} {:>3.0}%  {}",
        "Used:",
        used_value,
        mem_used_pct,
        bar(mem_used_pct, BarTone::Danger),
        value_width = value_width
    );
    let cache_line = format!(
        "  {:<6} {:<value_width$} {:>3.0}%  {}",
        "Cache:",
        cache_value,
        mem_cache_pct,
        bar(mem_cache_pct, BarTone::Warning),
        value_width = value_width
    );
    let avail_line = format!(
        "  {:<6} {:<value_width$} {:>3.0}%  {}",
        "Avail:",
        avail_value,
        mem_avail_pct,
        bar(mem_avail_pct, BarTone::Good),
        value_width = value_width
    );
    let swap_used_line = format!(
        "  {:<6} {:<value_width$} {:>3.0}%  {}",
        "Used:",
        swap_used_value,
        swap_pct,
        bar(swap_pct, BarTone::Danger),
        value_width = value_width
    );
    let swap_free_line = format!(
        "  {:<6} {:<value_width$} {:>3.0}%  {}",
        "Free:",
        swap_free_value,
        swap_free_pct,
        bar(swap_free_pct, BarTone::Good),
        value_width = value_width
    );

    format!(
        "\
Memory
{used_line}
{cache_line}
{avail_line}

Swap
{swap_used_line}
{swap_free_line}
",
        used_line = used_line,
        cache_line = cache_line,
        avail_line = avail_line,
        swap_used_line = swap_used_line,
        swap_free_line = swap_free_line,
    )
}

/// Create a colored progress bar.
fn bar(percent: f64, tone: BarTone) -> String {
    let filled = ((percent / 100.0) * BAR_WIDTH as f64).round() as usize;
    let empty = BAR_WIDTH.saturating_sub(filled);

    let filled_block = "█".repeat(filled);
    let empty_block = "░".repeat(empty);

    let raw_bar = format!("{filled_block}{empty_block}");

    match tone {
        BarTone::Danger => format!("\x1b[31m{raw_bar}\x1b[0m"),
        BarTone::Warning => format!("\x1b[33m{raw_bar}\x1b[0m"),
        BarTone::Good => format!("\x1b[32m{raw_bar}\x1b[0m"),
    }
}
