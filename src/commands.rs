use crate::fuzz::{create_fuzzlist, FuzzResult};
use anyhow::Result;
use colored::Colorize;
use futures::{stream, StreamExt};
use reqwest::Client;
use std::collections::HashSet;
use std::time::{Duration, Instant};

pub fn fuzz(target: &String, wordlist: &String, timeout: u64, concurrency: usize) -> Result<()> {
    // Benchmarking
    let fuzz_start = Instant::now();

    // Parse words from wordlist
    let wordlist = parse_wordlist(wordlist.as_str());

    // Create fuzzlist from target URL and wordlist
    let fuzzlist = create_fuzzlist(&target, wordlist)?;

    // Create new tokio runtime
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    // Create new reqwest HTTP client
    let http_timeout = Duration::from_secs(timeout);
    let http_client = Client::builder().timeout(http_timeout).build()?;

    // Create fuzz_results output vector
    let mut fuzz_results: Vec<FuzzResult> = Vec::new();

    log::info!("Fuzzing target URL");
    runtime.block_on(async {
        let _ = stream::iter(fuzzlist.into_iter())
            .map(|url| {
                let http_client = &http_client;
                async move {
                    match http_client.get(&url).send().await {
                        Ok(response) => {
                            // Get status code
                            let status_code = response.status().as_u16();

                            // Get the reason phrase.
                            let reason_phrase = response
                                .status()
                                .canonical_reason()
                                .map(|reason| reason.to_string())
                                .unwrap_or_else(|| response.status().to_string());

                            // Get the content length
                            let content_length = response.content_length();

                            FuzzResult {
                                url,
                                request_error: false,
                                status_code: Some(status_code),
                                reason_phrase: Some(reason_phrase),
                                content_length,
                            }
                        }
                        Err(err) => {
                            log::error!("{}: {}", url, err);
                            FuzzResult {
                                url,
                                request_error: true,
                                status_code: None,
                                reason_phrase: None,
                                content_length: None,
                            }
                        }
                    }
                }
            })
            .buffer_unordered(concurrency)
            .collect::<Vec<FuzzResult>>()
            .await
            .into_iter()
            .filter(|result| !result.request_error)
            .map(|result| {
                fuzz_results.push(result);
            })
            .collect::<Vec<_>>();
    });

    // Print output results
    println!("");
    for result in fuzz_results {
        let status_code = result.status_code.unwrap();

        // Colorise the status code based on type
        let status_code_colored = match status_code {
            // Informational responses
            100..=199 => status_code.to_string().on_yellow(),
            // Successful responses
            200..=299 => status_code.to_string().on_green(),
            // Redirection messages
            300..=399 => status_code.to_string().on_blue(),
            // Client error responses
            400..=499 => status_code.to_string().on_red(),
            // Server error responses
            500..=599 => status_code.to_string().on_bright_red(),

            // Other responses (should not happen)
            _ => status_code.to_string().on_white(),
        };

        let reason_phrase = result
            .reason_phrase
            .clone()
            .unwrap_or_else(|| "unknown".to_string());

        println!("{} {}: {}", status_code_colored, reason_phrase, result.url);
    }
    println!("");

    // Benchmarking
    log::info!("Fuzzing took: {:2?}", fuzz_start.elapsed());

    Ok(())
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
