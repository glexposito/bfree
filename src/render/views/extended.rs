use crate::core::memory_stats::MemoryStats;
use crate::render::Renderer;
use crate::render::format::fmt_short;

pub struct ExtendedView;

impl Renderer for ExtendedView {
    fn render(&self, s: &MemoryStats) -> String {
        let total = fmt_short(s.mem_total);
        let used = fmt_short(s.mem_used());
        let cache = fmt_short(s.mem_cache_effective());
        let avail = fmt_short(s.mem_available);

        let swap_total = fmt_short(s.swap_total);
        let swap_used = fmt_short(s.swap_used());
        let swap_free = fmt_short(s.swap_free);

        let cached = fmt_short(s.mem_cached);
        let sreclaimable = fmt_short(s.mem_sreclaimable);
        let shmem = fmt_short(s.mem_shmem);

        let used_pct = s.mem_used_percent();
        let cache_pct = s.mem_cache_percent();
        let avail_pct = s.mem_available_percent();
        let swap_used_pct = s.swap_used_percent();
        let swap_free_pct = s.swap_free_percent();

        format!(
            "\
memory  {total}
  used:       {used_pct:>3.0}%  {used}
  cache:      {cache_pct:>3.0}%  {cache}
  available:  {avail_pct:>3.0}%  {avail}

swap  {swap_total}
  used:  {swap_used_pct:>3.0}%  {swap_used}
  free:  {swap_free_pct:>3.0}%  {swap_free}

cache breakdown
  cached:        {cached}
  sreclaimable:  {sreclaimable}
  shmem:         {shmem}"
        )
    }
}
