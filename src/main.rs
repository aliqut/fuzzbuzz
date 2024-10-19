#![allow(unused)]
#![allow(dead_code)]

mod cli;
mod commands;
mod fuzz;
mod input;
mod output;

use crate::cli::Cli;
use anyhow::Result;
use clap::Parser;
use commands::fuzz;
use output::output_result;

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let cli = Cli::parse();

    let target = cli.target;
    let wordlist = cli.wordlist;
    let timeout = cli.timeout;
    let concurrency = cli.concurrency;

    // Fuzz the URL and store results
    let fuzz_results = fuzz(&target, &wordlist, timeout, concurrency)?;

    //Filter results based on CLI options and print to terminal output
    output_result(fuzz_results);

    Ok(())
}
