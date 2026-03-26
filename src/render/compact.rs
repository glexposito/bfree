use crate::core::memory_stats::MemoryStats;
use crate::render::Renderer;
use crate::render::format::fmt_short;

pub struct CompactRenderer {}

impl CompactRenderer {
    pub fn new() -> Self {
        Self {}
    }
}

/// Default one-line output (no colors, no styling).
///
/// Example:
/// Mem used 7.2GiB / 16GiB (45%) | Swap used 0.1GiB / 2GiB (5%)
impl Renderer for CompactRenderer {
    fn render(&self, stats: &MemoryStats) -> String {
        let used = fmt_short(stats.mem_used());
        let total = fmt_short(stats.mem_total);
        let swap_used = fmt_short(stats.swap_used());
        let swap_total = fmt_short(stats.swap_total);
        let used_pct = stats.mem_used_percent();
        let swap_used_pct = stats.swap_used_percent();

        format!(
            "Mem used {used} / {total} ({used_pct:.0}%) | Swap used {swap_used} / {swap_total} ({swap_used_pct:.0}%)"
        )
    }
}
