mod cli;
mod commands;
mod fuzz;
mod input;
mod output;

use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;
use commands::fuzz;
use output::output_result;

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
