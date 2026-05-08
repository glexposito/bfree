use crate::cli::Args;
use crate::core::memory_stats::MemoryStats;
use crate::render::structured::{self, StructuredFormat, StructuredView};
use crate::render::views::{CompactView, ExtendedView, PrettyView};
use crate::render::Renderer;

pub enum OutputMode {
    Compact,
    Extended,
    Pretty,
    Json(StructuredView),
    Yaml(StructuredView),
}

impl OutputMode {
    pub fn from_args(args: &Args) -> Self {
        let view = if args.extended { StructuredView::Extended } else { StructuredView::Compact };
        if args.json {
            Self::Json(view)
        } else if args.yaml {
            Self::Yaml(view)
        } else if args.pretty {
            Self::Pretty
        } else if args.extended {
            Self::Extended
        } else {
            Self::Compact
        }
    }

    pub fn render(self, stats: &MemoryStats) -> Result<String, String> {
        match self {
            Self::Compact  => Ok(CompactView.render(stats)),
            Self::Extended => Ok(ExtendedView.render(stats)),
            Self::Pretty   => Ok(PrettyView.render(stats)),
            Self::Json(view) => structured::render(stats, StructuredFormat::Json, view),
            Self::Yaml(view) => structured::render(stats, StructuredFormat::Yaml, view),
        }
    }
}
