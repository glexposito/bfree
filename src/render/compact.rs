use crate::core::memory_stats::MemoryStats;
use crate::render::format::fmt_short;

/// Default one-line output (no colors, no styling).
///
/// Example:
/// Mem used 7.2GiB / 16GiB (45%) | Swap used 0.1GiB / 2GiB (5%)
pub fn render(s: &MemoryStats) -> String {
    format!(
        "Mem used {} / {} ({:.0}%) | Swap used {} / {} ({:.0}%)",
        fmt_short(s.mem_used()),
        fmt_short(s.mem_total),
        s.mem_used_percent(),
        fmt_short(s.swap_used()),
        fmt_short(s.swap_total),
        s.swap_used_percent(),
    )
}
