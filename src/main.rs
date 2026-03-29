mod generator;

use clap::Parser;
use generator::{PasswordPolicy, generate_password};

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

fn validate_cli(cli: &Cli) -> Result<(), String> {
    let min_length = if cli.require_each_class { 4 } else { 1 };
    if cli.length < min_length {
        return Err(format!(
            "Password length must be at least {} when require_each_class is enabled",
            min_length
        ));
    }
    if cli.count == 0 {
        return Err("Count must be greater than 0".to_string());
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = validate_cli(&cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    let policy = PasswordPolicy {
        length: cli.length,
        include_upper: true,
        include_lower: true,
        include_digits: true,
        include_symbols: !cli.no_symbols,
        require_each_class: cli.require_each_class,
    };

    for _ in 0..cli.count {
        println!("{}", generate_password(&policy));
    }
}
