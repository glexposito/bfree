use crate::core::memory_stats::MemoryStats;

pub mod compact;
pub mod extended;
pub mod format;
pub mod presenter;
pub mod pretty;
pub mod structured;

pub trait Renderer {
    fn render(&self, stats: &MemoryStats) -> String;
}
