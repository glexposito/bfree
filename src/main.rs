use bfree::platform::linux;
use bfree::render::structured::{StructuredFormat, StructuredView};
use bfree::render::{compact, extended, pretty, structured};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "bfree - memory stats for humans")]

struct Args {
    #[arg(
        short = 'e',
        long = "extended",
        conflicts_with = "pretty",
        help = "Show multi-line details: totals, percentages, and cache breakdown"
    )]
    extended: bool,

    #[arg(
        short = 'p',
        long = "pretty",
        conflicts_with_all = ["extended", "json", "yaml"],
        help = "Show a visual multi-line view with colored bars"
    )]
    pretty: bool,

    #[arg(
        long = "json",
        conflicts_with = "yaml",
        help = "Render default (compact) or extended stats as pretty JSON"
    )]
    json: bool,

    #[arg(
        long = "yaml",
        conflicts_with = "json",
        help = "Render default (compact) or extended stats as YAML"
    )]
    yaml: bool,
}

enum OutputMode {
    Pretty,
    Json(StructuredView),
    Yaml(StructuredView),
    Extended,
    Compact,
}

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

    let mode = if args.pretty {
        OutputMode::Pretty
    } else if args.json {
        OutputMode::Json(view)
    } else if args.yaml {
        OutputMode::Yaml(view)
    } else if args.extended {
        OutputMode::Extended
    } else {
        OutputMode::Compact
    };

    match mode {
        OutputMode::Pretty => print!("{}", pretty::render(&stats)),
        OutputMode::Json(view) => {
            let output =
                structured::render(&stats, StructuredFormat::Json, view).unwrap_or_else(|e| {
                    eprintln!("bfree: {e}");
                    std::process::exit(1);
                });
            println!("{output}");
        }
        OutputMode::Yaml(view) => {
            let output =
                structured::render(&stats, StructuredFormat::Yaml, view).unwrap_or_else(|e| {
                    eprintln!("bfree: {e}");
                    std::process::exit(1);
                });
            println!("{output}");
        }
        OutputMode::Extended => print!("{}", extended::render(&stats)),
        OutputMode::Compact => println!("{}", compact::render(&stats)),
    }
}
