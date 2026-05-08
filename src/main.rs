use bfree::cli::Args;
use bfree::platform::linux;
use bfree::render::Renderer;
use bfree::render::presenter::{HumanFormat, Presenter};
use bfree::render::structured::{self, StructuredFormat, StructuredView};
use clap::Parser;

enum OutputMode {
    Human(HumanFormat),
    Structured(StructuredFormat, StructuredView),
}

impl OutputMode {
    fn from_args(args: &Args) -> Self {
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

    fn render(self, stats: &bfree::core::memory_stats::MemoryStats) -> Result<String, String> {
        match self {
            Self::Human(fmt) => Ok(Presenter::new(fmt).render(stats)),
            Self::Structured(fmt, view) => structured::render(stats, fmt, view),
        }
    }
}

fn main() {
    let args = Args::parse();

    let stats = linux::read_memory_stats().unwrap_or_else(|e| {
        eprintln!("bfree: {e}");
        std::process::exit(1);
    });

    let output = OutputMode::from_args(&args).render(&stats).unwrap_or_else(|e| {
        eprintln!("bfree: {e}");
        std::process::exit(1);
    });

    println!("{output}");
}
