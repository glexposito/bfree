use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about = "bfree - memory stats for humans")]
pub struct Args {
    #[arg(
        short = 'e',
        long = "extended",
        conflicts_with = "visual",
        help = "Show multi-line details: totals, percentages, and cache breakdown"
    )]
    pub extended: bool,

    #[arg(
        long = "visual",
        conflicts_with_all = ["extended", "json", "yaml"],
        help = "Show a visual multi-line view with monochrome bars"
    )]
    pub visual: bool,

    #[arg(
        long = "json",
        conflicts_with = "yaml",
        help = "Render default (compact) or extended stats as pretty JSON"
    )]
    pub json: bool,

    #[arg(
        long = "yaml",
        conflicts_with = "json",
        help = "Render default (compact) or extended stats as YAML"
    )]
    pub yaml: bool,
}
