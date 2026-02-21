use bfree::platform::linux;
use bfree::render::{compact, extended, pretty};
use clap::Parser;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "A better free, human by default.",
    long_about = None
)]

struct Args {
    #[arg(
        short = 'e',
        long = "extended",
        conflicts_with = "pretty",
        help = "Show extended memory details (totals, percentages, and kernel breakdown)"
    )]
    extended: bool,

    #[arg(
        short = 'p',
        long = "pretty",
        conflicts_with = "extended",
        help = "Show a visual view with colored bars for memory and swap"
    )]
    pretty: bool,
}

fn main() {
    let args = Args::parse();

    let stats = linux::read_memory_stats().unwrap_or_else(|e| {
        eprintln!("bfree: {e}");
        std::process::exit(1);
    });

    if args.pretty {
        print!("{}", pretty::render(&stats));
    } else if args.extended {
        print!("{}", extended::render(&stats));
    } else {
        println!("{}", compact::render(&stats));
    }
}
