use crate::core::memory_stats::MemoryStats;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructuredFormat {
    Json,
    Yaml,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructuredView {
    Compact,
    Extended,
}

#[derive(Serialize)]
struct PercentValue {
    percent: f64,
}

#[derive(Serialize)]
struct CompactMemory {
    total_bytes: u64,
    used_bytes: u64,
    used: PercentValue,
    available_bytes: u64,
    available: PercentValue,
}

#[derive(Serialize)]
struct CompactSwap {
    total_bytes: u64,
    used_bytes: u64,
    used: PercentValue,
}

#[derive(Serialize)]
struct CompactOutput {
    memory: CompactMemory,
    swap: CompactSwap,
}

#[derive(Serialize)]
struct ExtendedMemory {
    total_bytes: u64,
    used_bytes: u64,
    used: PercentValue,
    cache_bytes: u64,
    cache: PercentValue,
    available_bytes: u64,
    available: PercentValue,
}

#[derive(Serialize)]
struct ExtendedSwap {
    total_bytes: u64,
    used_bytes: u64,
    used: PercentValue,
    free_bytes: u64,
    free: PercentValue,
}

#[derive(Serialize)]
struct CacheBreakdown {
    cached_bytes: u64,
    sreclaimable_bytes: u64,
    shmem_bytes: u64,
}

#[derive(Serialize)]
struct ExtendedOutput {
    memory: ExtendedMemory,
    swap: ExtendedSwap,
    cache_breakdown: CacheBreakdown,
}

pub fn render(
    s: &MemoryStats,
    format: StructuredFormat,
    view: StructuredView,
) -> Result<String, String> {
    let content = match view {
        StructuredView::Compact => serialize_compact(s, format)?,
        StructuredView::Extended => serialize_extended(s, format)?,
    };
    Ok(content)
}

fn serialize_compact(s: &MemoryStats, format: StructuredFormat) -> Result<String, String> {
    let out = CompactOutput {
        memory: CompactMemory {
            total_bytes: s.mem_total,
            used_bytes: s.mem_used(),
            used: PercentValue {
                percent: s.mem_used_percent(),
            },
            available_bytes: s.mem_available,
            available: PercentValue {
                percent: s.mem_available_percent(),
            },
        },
        swap: CompactSwap {
            total_bytes: s.swap_total,
            used_bytes: s.swap_used(),
            used: PercentValue {
                percent: s.swap_used_percent(),
            },
        },
    };
    serialize(&out, format)
}

fn serialize_extended(s: &MemoryStats, format: StructuredFormat) -> Result<String, String> {
    let swap_free_pct = if s.swap_total == 0 {
        0.0
    } else {
        (s.swap_free as f64) * 100.0 / (s.swap_total as f64)
    };
    let out = ExtendedOutput {
        memory: ExtendedMemory {
            total_bytes: s.mem_total,
            used_bytes: s.mem_used(),
            used: PercentValue {
                percent: s.mem_used_percent(),
            },
            cache_bytes: s.mem_cache_effective(),
            cache: PercentValue {
                percent: s.mem_cache_percent(),
            },
            available_bytes: s.mem_available,
            available: PercentValue {
                percent: s.mem_available_percent(),
            },
        },
        swap: ExtendedSwap {
            total_bytes: s.swap_total,
            used_bytes: s.swap_used(),
            used: PercentValue {
                percent: s.swap_used_percent(),
            },
            free_bytes: s.swap_free,
            free: PercentValue {
                percent: swap_free_pct,
            },
        },
        cache_breakdown: CacheBreakdown {
            cached_bytes: s.mem_cached,
            sreclaimable_bytes: s.mem_sreclaimable,
            shmem_bytes: s.mem_shmem,
        },
    };
    serialize(&out, format)
}

fn serialize<T: Serialize>(value: &T, format: StructuredFormat) -> Result<String, String> {
    match format {
        StructuredFormat::Json => serde_json::to_string_pretty(value)
            .map_err(|e| format!("json serialization failed: {e}")),
        StructuredFormat::Yaml => {
            serde_yaml::to_string(value).map_err(|e| format!("yaml serialization failed: {e}"))
        }
    }
}
