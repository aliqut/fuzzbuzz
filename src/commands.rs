use crate::{
    fuzz::{create_fuzzlist, FuzzResult},
    input::parse_wordlist,
};
use anyhow::Result;
use futures::{stream, StreamExt};
use reqwest::Client;
use std::time::{Duration, Instant};

pub fn fuzz(
    target: &String,
    wordlist: &String,
    timeout: u64,
    concurrency: usize,
) -> Result<Vec<FuzzResult>> {
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

    // Benchmarking
    log::info!("Fuzzing took: {:2?}", fuzz_start.elapsed());

    Ok(fuzz_results)
}
