use bfree::platform::linux;
use bfree::render::{text, verbose};
use clap::Parser;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "A better free, human by default.",
    long_about = None
)]
struct Args {
    /// Show extended memory breakdown
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let stats = linux::read_memory_stats().unwrap_or_else(|e| {
        eprintln!("bfree: {e}");
        std::process::exit(1);
    });

    if args.verbose {
        // verbose::render() already includes a trailing newline in the format string,
        // so we print without adding another one.
        print!("{}", verbose::render(&stats));
    } else {
        println!("{}", text::one_line(&stats));
    }
}
