use crate::core::memory_stats::MemoryStats;
use crate::render::Renderer;
use crate::render::views::{CompactView, ExtendedView, PrettyView};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HumanFormat {
    Compact,
    Extended,
    Pretty,
}

pub struct Presenter {
    format: HumanFormat,
}

impl Presenter {
    pub fn new(format: HumanFormat) -> Self {
        Self { format }
    }
}

impl Renderer for Presenter {
    fn render(&self, stats: &MemoryStats) -> String {
        match self.format {
            HumanFormat::Compact => CompactView::default().render(stats),
            HumanFormat::Extended => ExtendedView::default().render(stats),
            HumanFormat::Pretty => PrettyView::default().render(stats),
        }
    }
}
