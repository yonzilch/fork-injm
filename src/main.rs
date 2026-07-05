mod cli;
mod detector;

use clap::Parser;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();
    let lang = detector::detect(&cli.output)?;
    println!("language is {}", lang);

    Ok(())
}
