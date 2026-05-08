use crate::core::memory_stats::MemoryStats;

pub mod format;
pub mod structured;
pub mod views;

pub trait Renderer {
    fn render(&self, stats: &MemoryStats) -> String;
}
