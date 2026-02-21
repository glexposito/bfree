use bfree::platform::linux;
use bfree::render::{pretty, text, verbose};
use clap::Parser;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "A better free, human by default.",
    long_about = None
)]

struct Args {
    #[arg(short, long)]
    verbose: bool,

    #[arg(long)]
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
    } else if args.verbose {
        print!("{}", verbose::render(&stats));
    } else {
        println!("{}", text::one_line(&stats));
    }
}
