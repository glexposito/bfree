use crate::core::memory_stats::MemoryStats;
use crate::render::format::fmt_short;

/// Default one-line output (no colors, no styling).
///
/// Example:
/// Mem 16.0G used 7.2G (45%) cache 5.8G (36%) avail 8.8G (55%) | Swap 2.0G used 0.1G (5%)
pub fn render(s: &MemoryStats) -> String {
    format!(
        "Mem {} used {} ({:.0}%) cache {} ({:.0}%) avail {} ({:.0}%) | Swap {} used {} ({:.0}%)",
        fmt_short(s.mem_total),
        fmt_short(s.mem_used()),
        s.mem_used_percent(),
        fmt_short(s.mem_cache_effective()),
        s.mem_cache_percent(),
        fmt_short(s.mem_available),
        s.mem_available_percent(),
        fmt_short(s.swap_total),
        fmt_short(s.swap_used()),
        s.swap_used_percent(),
    )
}
