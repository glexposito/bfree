use bfree::cli::Args;
use bfree::platform::linux;
use bfree::render::structured::{StructuredFormat, StructuredView};
use bfree::render::{compact, extended, pretty, structured};
use clap::Parser;

fn main() {
    let args = Args::parse();

    let stats = linux::read_memory_stats().unwrap_or_else(|e| {
        eprintln!("bfree: {e}");
        std::process::exit(1);
    });

    let view = if args.extended {
        StructuredView::Extended
    } else {
        StructuredView::Compact
    };

    let output = if args.visual {
        pretty::render(&stats)
    } else if args.json {
        structured::render(&stats, StructuredFormat::Json, view).unwrap_or_else(|e| {
            eprintln!("bfree: {e}");
            std::process::exit(1);
        })
    } else if args.yaml {
        structured::render(&stats, StructuredFormat::Yaml, view).unwrap_or_else(|e| {
            eprintln!("bfree: {e}");
            std::process::exit(1);
        })
    } else if args.extended {
        extended::render(&stats)
    } else {
        compact::render(&stats)
    };

    println!("{output}");
}
