use std::time::Instant;

use crate::{
    fuzz::{create_fuzzlist, FuzzResult},
    parse_wordlist,
};
use anyhow::Result;
use futures::{stream, StreamExt};
use reqwest::Client;

pub fn fuzz(target: &String, wordlist: &String) -> Result<()> {
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

    // Number of threads
    let concurrency = 200;

    // Create new reqwest HTTP client
    let http_client = Client::new();

    // Create fuzz_results output vector
    let mut fuzz_results: Vec<FuzzResult> = Vec::new();

    runtime.block_on(async {
        let _ = stream::iter(fuzzlist.into_iter())
            .map(|url| {
                let http_client = &http_client;
                async move {
                    log::info!("Checking: {}", &url);

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
                            FuzzResult {
                                url,
                                request_error: false,
                                status_code: Some(status_code),
                                reason_phrase: Some(reason_phrase),
                            }
                        }
                        Err(err) => {
                            log::error!("{}: {}", url, err);
                            FuzzResult {
                                url,
                                request_error: true,
                                status_code: None,
                                reason_phrase: None,
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

    dbg!(fuzz_results);

    // Benchmarking
    log::info!("Fuzzing took: {:2?}", fuzz_start.elapsed());

    Ok(())
}
