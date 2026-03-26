use crate::core::memory_stats::MemoryStats;
use crate::render::Renderer;
use crate::render::compact::CompactRenderer;
use crate::render::extended::ExtendedRenderer;
use crate::render::pretty::PrettyRenderer;

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
            HumanFormat::Compact => CompactRenderer::new().render(stats),
            HumanFormat::Extended => ExtendedRenderer::new().render(stats),
            HumanFormat::Pretty => PrettyRenderer::new().render(stats),
        }
    }
}
