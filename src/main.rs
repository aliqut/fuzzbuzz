mod commands;
mod fuzz;
mod output;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::fuzz;
use output::output_result;

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

        /// HTTP request timeout length in seconds
        #[arg(short, long, default_value_t = 4)]
        timeout: u64,

        /// Number of concurrent threads
        #[arg(short, long, default_value_t = 200)]
        concurrency: usize,
    },
}

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let cli = Cli::parse();
    match &cli.command {
        Commands::Fuzz {
            target,
            wordlist,
            timeout,
            concurrency,
        } => {
            let fuzz_results = fuzz(target, wordlist, *timeout, *concurrency)?;
            output_result(fuzz_results);
            return Ok(());
        }
    }
}
