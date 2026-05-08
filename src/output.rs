use crate::cli::Args;
use crate::core::memory_stats::MemoryStats;
use crate::render::presenter::{HumanFormat, Presenter};
use crate::render::structured::{self, StructuredFormat, StructuredView};
use crate::render::Renderer;

pub enum OutputMode {
    Human(HumanFormat),
    Structured(StructuredFormat, StructuredView),
}

impl OutputMode {
    pub fn from_args(args: &Args) -> Self {
        let view = if args.extended { StructuredView::Extended } else { StructuredView::Compact };
        if args.json {
            Self::Structured(StructuredFormat::Json, view)
        } else if args.yaml {
            Self::Structured(StructuredFormat::Yaml, view)
        } else {
            let fmt = match (args.visual, args.extended) {
                (true, _) => HumanFormat::Pretty,
                (_, true) => HumanFormat::Extended,
                _ => HumanFormat::Compact,
            };
            Self::Human(fmt)
        }
    }

    pub fn render(self, stats: &MemoryStats) -> Result<String, String> {
        match self {
            Self::Human(fmt) => Ok(Presenter::new(fmt).render(stats)),
            Self::Structured(fmt, view) => structured::render(stats, fmt, view),
        }
    }
}
