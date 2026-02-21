use bfree::platform::linux;
use bfree::render::text;
use clap::Parser;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "A better free, human by default.",
    long_about = None
)]

struct Args {}

fn main() {
    // Enables --help and --version
    let _ = Args::parse();

    // Real logic
    let stats = linux::read_memory_stats().unwrap_or_else(|e| {
        eprintln!("bfree: {e}");
        std::process::exit(1);
    });

    println!("{}", text::one_line(&stats));
}
