use anyhow::{Context, Result};
use std::collections::HashSet;

pub fn parse_wordlist(file_path: &str) -> Result<Vec<String>> {
    // Read file into string
    let wordlist_file = std::fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read the wordlist file: {}", &file_path))?;

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

    Ok(wordlist)
}
