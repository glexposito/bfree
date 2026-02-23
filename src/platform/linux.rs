use crate::core::memory_stats::MemoryStats;
use procfs::Current;

#[derive(Debug)]
pub struct LinuxMemError(procfs::ProcError);

/// Linux memory snapshot exposed by this library.
///
/// Values are normalized to bytes and optional kernel fields are defaulted to 0.
#[derive(Debug, Clone, Copy)]
pub struct LinuxMemSnapshot {
    pub mem_total: u64,
    pub mem_available: u64,
    pub mem_cached: u64,
    pub mem_sreclaimable: u64,
    pub mem_shmem: u64,
    pub swap_total: u64,
    pub swap_free: u64,
}

/// Read memory stats from Linux `/proc/meminfo` and return a typed domain model.
///
/// This is intentionally Linux-specific. Callers get `MemoryStats` and do not
/// need to know anything about `/proc` keys or parsing.
pub fn read_memory_stats() -> Result<MemoryStats, LinuxMemError> {
    let mem = read_mem_info()?;
    Ok(map_mem_info_to_stats(mem))
}

/// Read Linux memory information and return the library-owned model.
fn read_mem_info() -> Result<LinuxMemSnapshot, LinuxMemError> {
    let mem = procfs::Meminfo::current().map_err(LinuxMemError)?;

    Ok(LinuxMemSnapshot {
        mem_total: mem.mem_total,
        mem_available: mem.mem_available.unwrap_or(mem.mem_free),
        mem_cached: mem.cached,
        mem_sreclaimable: mem.s_reclaimable.unwrap_or(0),
        mem_shmem: mem.shmem.unwrap_or(0),
        swap_total: mem.swap_total,
        swap_free: mem.swap_free,
    })
}

fn map_mem_info_to_stats(mem: LinuxMemSnapshot) -> MemoryStats {
    MemoryStats::new(
        mem.mem_total,
        mem.mem_available,
        mem.mem_cached,
        mem.mem_sreclaimable,
        mem.mem_shmem,
        mem.swap_total,
        mem.swap_free,
    )
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

#[cfg(test)]
mod tests {
    use super::{LinuxMemSnapshot, map_mem_info_to_stats};

    #[test]
    fn map_mem_info_to_stats_maps_fields_1_to_1() {
        let stats = map_mem_info_to_stats(LinuxMemSnapshot {
            mem_total: 1000,
            mem_available: 600,
            mem_cached: 100,
            mem_sreclaimable: 50,
            mem_shmem: 20,
            swap_total: 300,
            swap_free: 250,
        });

        assert_eq!(stats.mem_total, 1000);
        assert_eq!(stats.mem_available, 600);
        assert_eq!(stats.mem_cached, 100);
        assert_eq!(stats.mem_sreclaimable, 50);
        assert_eq!(stats.mem_shmem, 20);
        assert_eq!(stats.swap_total, 300);
        assert_eq!(stats.swap_free, 250);
    }
}
