//#![allow(unused)]
//#![allow(dead_code)]

mod cli;
mod filters;
mod fuzz;
mod http;
mod input;
mod output;

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

    let target = cli.target;
    let wordlist = cli.wordlist;
    let timeout = cli.timeout;
    let concurrency = cli.concurrency;
    let filter_status = cli.filter_status;
    let filter_size = cli.filter_size;
    let filter_lines = cli.filter_lines;
    let match_status = cli.match_status;
    let match_size = cli.match_size;
    let match_lines = cli.match_lines;

    // Parse filters from CLI options
    let response_filters = parse_response_filters(
        match_status,
        match_size,
        match_lines,
        filter_status,
        filter_size,
        filter_lines,
    );

    // Parse HTTP request options from CLI options
    let header_map = parse_headers(cli.headers, cli.cookies);

    // Create HTTP client
    let http_client = create_http_client(timeout, header_map)?;

    // Fuzz the URL and store results
    let fuzz_responses = fuzz(
        &target,
        &wordlist,
        http_client,
        concurrency,
        response_filters,
    )?;

    //Filter results based on CLI options and print to terminal output
    output_result(fuzz_responses);

    Ok(())
}
