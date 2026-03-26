use crate::core::memory_stats::MemoryStats;
use crate::render::structured::PercentValue;
use serde::Serialize;

#[derive(Serialize)]
pub(super) struct CompactMemory {
    total_bytes: u64,
    used_bytes: u64,
    used: PercentValue,
    available_bytes: u64,
    available: PercentValue,
}

#[derive(Serialize)]
pub(super) struct CompactSwap {
    total_bytes: u64,
    used_bytes: u64,
    used: PercentValue,
}

#[derive(Serialize)]
pub(super) struct CompactOutput {
    memory: CompactMemory,
    swap: CompactSwap,
}

impl CompactOutput {
    pub(super) fn from_stats(stats: &MemoryStats) -> Self {
        Self {
            memory: CompactMemory {
                total_bytes: stats.mem_total,
                used_bytes: stats.mem_used(),
                used: PercentValue::new(stats.mem_used_percent()),
                available_bytes: stats.mem_available,
                available: PercentValue::new(stats.mem_available_percent()),
            },
            swap: CompactSwap {
                total_bytes: stats.swap_total,
                used_bytes: stats.swap_used(),
                used: PercentValue::new(stats.swap_used_percent()),
            },
        }
    }
}
