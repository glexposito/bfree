use crate::core::memory_stats::MemoryStats;
use crate::render::Renderer;
use crate::render::compact::CompactView;
use crate::render::extended::ExtendedView;
use crate::render::pretty::PrettyView;

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

    pub fn render(&self, stats: &MemoryStats) -> String {
        match self.format {
            HumanFormat::Compact => CompactView::new().render(stats),
            HumanFormat::Extended => ExtendedView::new().render(stats),
            HumanFormat::Pretty => PrettyView::new().render(stats),
        }
    }
}
