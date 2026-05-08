use bfree::cli::Args;
use bfree::output::OutputMode;
use bfree::platform::linux;
use clap::Parser;

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
