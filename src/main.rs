use bfree::cli::Args;
use bfree::platform::linux;
use bfree::render::presenter::{HumanFormat, Presenter};
use bfree::render::structured;
use bfree::render::structured::{StructuredFormat, StructuredView};
use clap::Parser;

enum OutputKind {
    Human,
    Json,
    Yaml,
}

fn output_kind(args: &Args) -> OutputKind {
    if args.json {
        OutputKind::Json
    } else if args.yaml {
        OutputKind::Yaml
    } else {
        OutputKind::Human
    }
}

fn structured_view(args: &Args) -> StructuredView {
    match args.extended {
        true => StructuredView::Extended,
        false => StructuredView::Compact,
    }
}

fn human_format(args: &Args) -> HumanFormat {
    match (args.visual, args.extended) {
        (true, false) => HumanFormat::Pretty,
        (false, true) => HumanFormat::Extended,
        (false, false) => HumanFormat::Compact,
        (true, true) => unreachable!("clap prevents --visual and --extended together"),
    }
}

fn main() {
    let args = Args::parse();

    let stats = linux::read_memory_stats().unwrap_or_else(|e| {
        eprintln!("bfree: {e}");
        std::process::exit(1);
    });

    let output = match output_kind(&args) {
        OutputKind::Json => {
            structured::render(&stats, StructuredFormat::Json, structured_view(&args))
                .unwrap_or_else(|e| {
                    eprintln!("bfree: {e}");
                    std::process::exit(1);
                })
        }
        OutputKind::Yaml => {
            structured::render(&stats, StructuredFormat::Yaml, structured_view(&args))
                .unwrap_or_else(|e| {
                    eprintln!("bfree: {e}");
                    std::process::exit(1);
                })
        }
        OutputKind::Human => Presenter::new(human_format(&args)).render(&stats),
    };

    println!("{output}");
}
