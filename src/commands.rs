use crate::{
    filters::ResponseFilters,
    fuzz::{create_fuzzlist, FuzzResponse},
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
    response_filters: ResponseFilters,
) -> Result<Vec<FuzzResponse>> {
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

    // Create fuzz_responses output vector
    let mut fuzz_responses: Vec<FuzzResponse> = Vec::new();

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

                            // Read response body
                            let body = response.text().await.unwrap_or_else(|_| "".to_string());

                            FuzzResponse {
                                url,
                                body,
                                request_error: false,
                                status_code: Some(status_code),
                                reason_phrase: Some(reason_phrase),
                                content_length,
                            }
                        }
                        Err(err) => {
                            log::error!("{}: {}", url, err);
                            FuzzResponse {
                                url,
                                body: "".to_string(),
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
            .collect::<Vec<FuzzResponse>>()
            .await
            .into_iter()
            // Filter out results with reqwest errors
            .filter(|result| !result.request_error)
            // Apply status code filter
            .filter(|result| {
                if let Some(ref allowed_status) = response_filters.status_filters {
                    result
                        .status_code
                        .map_or(false, |status| allowed_status.contains(&status.to_string()))
                } else {
                    true
                }
            })
            // Apply content length filter
            .filter(|result| {
                if let Some(ref size_ranges) = response_filters.size_filters {
                    if let Some(content_length) = result.content_length {
                        size_ranges.iter().any(|(min, max)| {
                            content_length >= (*min).try_into().unwrap()
                                && content_length <= (*max).try_into().unwrap()
                        })
                    } else {
                        false
                    }
                } else {
                    true
                }
            })
            // Apply line count filter
            .filter(|result| {
                if let Some(ref line_ranges) = response_filters.line_filters {
                    let line_count = result.body.lines().count() as usize;
                    line_ranges
                        .iter()
                        .any(|(min, max)| line_count >= *min && line_count <= *max)
                } else {
                    true
                }
            })
            .map(|result| {
                fuzz_responses.push(result);
            })
            .collect::<Vec<_>>();
    });

    // Benchmarking
    log::info!("Fuzzing took: {:2?}", fuzz_start.elapsed());

    Ok(fuzz_responses)
}
