mod commands;
mod fuzz;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::fuzz;
use std::{collections::HashSet, env};

#[derive(Parser)]
#[command(name = "furl")]
#[command(author = "aliqut")]
#[command(version = "0.1.0")]
#[command(about = "Web fuzzer written in Rust")]
struct Cli {
    /// Subcommand to run
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fuzz the target URL
    Fuzz {
        /// The target URL. (e.g., http://example.com/FUZZ)
        target: String,

        /// Path to a wordlist file
        wordlist: String,
    },
}

fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let cli = Cli::parse();
    match &cli.command {
        Commands::Fuzz { target, wordlist } => {
            fuzz(target, wordlist)?;
            return Ok(());
        }
    }
}

fn parse_wordlist(file_path: &str) -> Vec<String> {
    // Read file into string
    let wordlist_file = std::fs::read_to_string(file_path).unwrap();

    // Split wordlist by newline & whitespace. Remove duplicates & empty lines.
    let wordlist: HashSet<String> = wordlist_file
        .split_whitespace()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|word| word.trim().to_string())
        .filter(|word| !word.is_empty())
        .collect();

    // Convert wordlist from HashSet to Vec
    let wordlist: Vec<String> = wordlist.into_iter().collect();

    wordlist
}
