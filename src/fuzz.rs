use anyhow::Result;

pub fn create_fuzzlist(target_url: &str, wordlist: Vec<String>) -> Result<Vec<String>> {
    // Check if URL contains FUZZ (FUZZ will be replaced by words in the wordlist)
    if !target_url.contains("FUZZ") {
        log::error!("Invalid target URL. Must contain \"FUZZ\"");
    }

    // For each word in wordlist, replace FUZZ in target_url with the word, output as a string
    // vector
    let fuzzlist: Vec<String> = wordlist
        .into_iter()
        .map(|word| target_url.replace("FUZZ", word.as_str()))
        .collect();

    Ok(fuzzlist)
}

#[derive(Debug)]
pub struct FuzzResult {
    pub url: String,
    pub request_error: bool,
    pub status_code: Option<u16>,
    pub reason_phrase: Option<String>,
}
