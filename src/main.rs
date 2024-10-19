mod commands;
mod fuzz;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::fuzz;

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
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let cli = Cli::parse();
    match &cli.command {
        Commands::Fuzz { target, wordlist } => {
            fuzz(target, wordlist)?;
            return Ok(());
        }
    }
}
