//#![allow(unused)]
//#![allow(dead_code)]

mod cli;
mod filters;
mod fuzz;
mod http;
mod input;
mod output;
//mod tests;

use crate::cli::Cli;
use anyhow::Result;
use clap::Parser;
use filters::parse_response_filters;
use fuzz::fuzz;
use http::{create_http_client, parse_headers};
use output::output_result;

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let cli = Cli::parse();

    // Parse filters from CLI options
    let response_filters = parse_response_filters(
        cli.match_status,
        cli.match_size,
        cli.match_lines,
        cli.filter_status,
        cli.filter_size,
        cli.filter_lines,
    );

    // Parse HTTP request options from CLI options
    let header_map = parse_headers(cli.headers, cli.cookies);

    // Create HTTP client
    let http_client = create_http_client(cli.timeout, header_map, cli.redirects)?;

    // Fuzz the URL and store results
    let fuzz_responses = fuzz(
        &cli.target,
        &cli.wordlist,
        http_client,
        cli.concurrency,
        response_filters?,
        cli.delay,
    )?;

    //Filter results based on CLI options and print to terminal output
    output_result(fuzz_responses);

    Ok(())
}
