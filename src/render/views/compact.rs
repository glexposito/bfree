use crate::core::memory_stats::MemoryStats;
use crate::render::Renderer;
use crate::render::format::fmt_short;

pub struct CompactView;

impl Renderer for CompactView {
    fn render(&self, stats: &MemoryStats) -> String {
        let used = fmt_short(stats.mem_used());
        let total = fmt_short(stats.mem_total);
        let swap_used = fmt_short(stats.swap_used());
        let swap_total = fmt_short(stats.swap_total);
        let used_pct = stats.mem_used_percent();
        let swap_used_pct = stats.swap_used_percent();
        let w = used.len().max(swap_used.len());

        format!(
            "mem   {used_pct:>3.0}%  {used:<w$} / {total}\nswap  {swap_used_pct:>3.0}%  {swap_used:<w$} / {swap_total}"
        )
    }
}
