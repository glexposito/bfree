#[derive(Debug, Clone)]
pub struct MemoryStats {
    // Memory
    pub mem_total: u64,
    pub mem_available: u64,
    pub mem_cached: u64,       // Cached
    pub mem_sreclaimable: u64, // SReclaimable
    pub mem_shmem: u64,        // Shmem

    // Swap
    pub swap_total: u64,
    pub swap_free: u64,
}

impl MemoryStats {
    pub fn new(
        mem_total: u64,
        mem_available: u64,
        mem_cached: u64,
        mem_sreclaimable: u64,
        mem_shmem: u64,
        swap_total: u64,
        swap_free: u64,
    ) -> Self {
        Self {
            mem_total,
            mem_available,
            mem_cached,
            mem_sreclaimable,
            mem_shmem,
            swap_total,
            swap_free,
        }
    }

    pub fn mem_used(&self) -> u64 {
        self.mem_total.saturating_sub(self.mem_available)
    }

    /// Effective reclaimable cache:
    /// cache ≈ Cached + SReclaimable - Shmem
    pub fn mem_cache_effective(&self) -> u64 {
        self.mem_cached
            .saturating_add(self.mem_sreclaimable)
            .saturating_sub(self.mem_shmem)
    }

    pub fn mem_used_percent(&self) -> f64 {
        percent(self.mem_used(), self.mem_total)
    }

    pub fn mem_cache_percent(&self) -> f64 {
        percent(self.mem_cache_effective(), self.mem_total)
    }

    pub fn mem_available_percent(&self) -> f64 {
        percent(self.mem_available, self.mem_total)
    }

    pub fn swap_used(&self) -> u64 {
        self.swap_total.saturating_sub(self.swap_free)
    }

    pub fn swap_used_percent(&self) -> f64 {
        if self.swap_total == 0 {
            0.0
        } else {
            percent(self.swap_used(), self.swap_total)
        }
    }
}

fn percent(n: u64, d: u64) -> f64 {
    if d == 0 {
        0.0
    } else {
        (n as f64) * 100.0 / (d as f64)
    }
}
