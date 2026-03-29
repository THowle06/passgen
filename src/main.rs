mod generator;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Length of each password
    #[arg(short, long, default_value_t = 16)]
    length: usize,

    /// Number of passwords to generate
    #[arg(short, long, default_value_t = 1)]
    count: usize,

    /// Exclude symbols from generated passwords
    #[arg(long)]
    no_symbols: bool,

    /// Ensure each enabled class appears at least once
    #[arg(long)]
    require_each_class: bool,

    /// Copy output to clipboard
    #[arg(long)]
    clipboard: bool,
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}
