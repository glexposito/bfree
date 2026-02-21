use crate::core::memory_stats::MemoryStats;

/// Default one-line output (no colors, no styling).
///
/// Example:
/// Mem 16.0G used 7.2G (45%) cache 5.8G (36%) avail 8.8G (55%) | Swap 2.0G used 0.1G (5%)
pub fn one_line(s: &MemoryStats) -> String {
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

/// Format bytes into short human-readable units (binary/IEC style).
/// 1024 base, like Linux tools.
fn fmt_short(bytes: u64) -> String {
    const K: f64 = 1024.0;
    const M: f64 = 1024.0 * 1024.0;
    const G: f64 = 1024.0 * 1024.0 * 1024.0;
    const T: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;

    let b = bytes as f64;

    if b >= T {
        format!("{:.1}T", b / T)
    } else if b >= G {
        format!("{:.1}G", b / G)
    } else if b >= M {
        format!("{:.1}M", b / M)
    } else if b >= K {
        format!("{:.0}K", b / K)
    } else {
        format!("{bytes}B")
    }
}
