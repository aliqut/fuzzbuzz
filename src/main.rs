//#![allow(unused)]
//#![allow(dead_code)]

mod cli;
mod filters;
mod fuzz;
mod input;
mod output;

use crate::cli::Cli;
use anyhow::Result;
use clap::Parser;
use filters::parse_response_filters;
use fuzz::fuzz;
use output::output_result;

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let cli = Cli::parse();

    let target = cli.target.clone();
    let wordlist = cli.wordlist.clone();
    let timeout = cli.timeout;
    let concurrency = cli.concurrency;

    // Parse filters from CLI options
    let response_filters = parse_response_filters(cli);

    // Fuzz the URL and store results
    let fuzz_responses = fuzz(&target, &wordlist, timeout, concurrency, response_filters)?;

    //Filter results based on CLI options and print to terminal output
    output_result(fuzz_responses);

    Ok(())
}
