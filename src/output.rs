use crate::fuzz::FuzzResult;
use colored::Colorize;

pub fn output_result(fuzz_results: Vec<FuzzResult>) {
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

        let content_length = result.content_length.unwrap_or_else(|| 0);

        println!(
            "{} {}, Size: {} : {}",
            status_code_colored, reason_phrase, content_length, result.url
        );
    }
    println!("");
}
