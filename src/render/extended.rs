use crate::core::memory_stats::MemoryStats;
use crate::render::format::fmt_short;

/// Extended, multi-line output with an extra kernel breakdown section.
pub fn render(s: &MemoryStats) -> String {
    format!(
        "\
Memory
  Total:        {total}
  Used:         {used} ({used_pct:.0}%)
  Cache:        {cache} ({cache_pct:.0}%)
  Available:    {avail} ({avail_pct:.0}%)

Swap
  Total:        {swap_total}
  Used:         {swap_used} ({swap_pct:.0}%)

Kernel Breakdown
  Cached:        {cached}
  SReclaimable:  {sreclaimable}
  Shmem:         {shmem}
",
        total = fmt_short(s.mem_total),
        used = fmt_short(s.mem_used()),
        used_pct = s.mem_used_percent(),
        cache = fmt_short(s.mem_cache_effective()),
        cache_pct = s.mem_cache_percent(),
        avail = fmt_short(s.mem_available),
        avail_pct = s.mem_available_percent(),
        swap_total = fmt_short(s.swap_total),
        swap_used = fmt_short(s.swap_used()),
        swap_pct = s.swap_used_percent(),
        cached = fmt_short(s.mem_cached),
        sreclaimable = fmt_short(s.mem_sreclaimable),
        shmem = fmt_short(s.mem_shmem),
    )
}
