use crate::core::memory_stats::MemoryStats;
use crate::render::format::fmt_short;

/// Default one-line output (no colors, no styling).
///
/// Example:
/// Mem used 7.2GiB / 16GiB (45%) | Swap used 0.1GiB / 2GiB (5%)
pub fn render(s: &MemoryStats) -> String {
    let used = fmt_short(s.mem_used());
    let total = fmt_short(s.mem_total);
    let swap_used = fmt_short(s.swap_used());
    let swap_total = fmt_short(s.swap_total);
    let used_pct = s.mem_used_percent();
    let swap_used_pct = s.swap_used_percent();

    format!(
        "Mem used {used} / {total} ({used_pct:.0}%) | Swap used {swap_used} / {swap_total} ({swap_used_pct:.0}%)"
    )
}
