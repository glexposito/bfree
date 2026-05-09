use crate::core::memory_stats::MemoryStats;
use procfs::Current;

#[derive(Debug)]
pub struct LinuxMemError(procfs::ProcError);

/// Read memory stats from Linux `/proc/meminfo` and return a typed domain model.
///
/// This is intentionally Linux-specific. Callers get `MemoryStats` and do not
/// need to know anything about `/proc` keys or parsing.
pub fn read_memory_stats() -> Result<MemoryStats, LinuxMemError> {
    let mem = procfs::Meminfo::current().map_err(LinuxMemError)?;

    Ok(MemoryStats::new(
        mem.mem_total,
        mem.mem_available.unwrap_or(mem.mem_free),
        mem.cached,
        mem.s_reclaimable.unwrap_or(0),
        mem.shmem.unwrap_or(0),
        mem.swap_total,
        mem.swap_free,
    ))
}

impl std::fmt::Display for LinuxMemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to read /proc/meminfo: {}", self.0)
    }
}

impl std::error::Error for LinuxMemError {}

impl From<procfs::ProcError> for LinuxMemError {
    fn from(value: procfs::ProcError) -> Self {
        Self(value)
    }
}
