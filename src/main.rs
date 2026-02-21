use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    watch: Option<f64>,

    #[arg(long)]
    json: bool,

    #[arg(long)]
    no_color: bool,

    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    // Parse CLI arguments
    let args = Args::parse();

    println!("Parsed arguments:");
    println!("{:#?}", args);

    if let Some(interval) = args.watch {
        println!("Watch mode enabled: refreshing every {} seconds", interval);
    }

    if args.json {
        println!("JSON output mode enabled");
    }

    if args.no_color {
        println!("Color disabled");
    }

    if args.verbose {
        println!("Verbose mode enabled");
    }

    println!("(No memory logic implemented yet)");
}
