use crate::core::memory_stats::MemoryStats;
use crate::render::structured::PercentValue;
use serde::Serialize;

#[derive(Serialize)]
pub(super) struct ExtendedMemory {
    total_bytes: u64,
    used_bytes: u64,
    used: PercentValue,
    cache_bytes: u64,
    cache: PercentValue,
    available_bytes: u64,
    available: PercentValue,
}

#[derive(Serialize)]
pub(super) struct ExtendedSwap {
    total_bytes: u64,
    used_bytes: u64,
    used: PercentValue,
    free_bytes: u64,
    free: PercentValue,
}

#[derive(Serialize)]
pub(super) struct CacheBreakdown {
    cached_bytes: u64,
    sreclaimable_bytes: u64,
    shmem_bytes: u64,
}

#[derive(Serialize)]
pub(super) struct ExtendedOutput {
    memory: ExtendedMemory,
    swap: ExtendedSwap,
    cache_breakdown: CacheBreakdown,
}

impl From<&MemoryStats> for ExtendedOutput {
    fn from(stats: &MemoryStats) -> Self {
        Self {
            memory: ExtendedMemory {
                total_bytes: stats.mem_total,
                used_bytes: stats.mem_used(),
                used: PercentValue::from(stats.mem_used_percent()),
                cache_bytes: stats.mem_cache_effective(),
                cache: PercentValue::from(stats.mem_cache_percent()),
                available_bytes: stats.mem_available,
                available: PercentValue::from(stats.mem_available_percent()),
            },
            swap: ExtendedSwap {
                total_bytes: stats.swap_total,
                used_bytes: stats.swap_used(),
                used: PercentValue::from(stats.swap_used_percent()),
                free_bytes: stats.swap_free,
                free: PercentValue::from(stats.swap_free_percent()),
            },
            cache_breakdown: CacheBreakdown {
                cached_bytes: stats.mem_cached,
                sreclaimable_bytes: stats.mem_sreclaimable,
                shmem_bytes: stats.mem_shmem,
            },
        }
    }
}
