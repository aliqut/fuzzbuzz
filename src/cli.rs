use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "furl")]
#[command(author = "aliqut")]
#[command(version = "0.1.0")]
#[command(about = "Web fuzzer written in Rust")]
pub struct Cli {
    /// Subcommand to run
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Fuzz the target URL
    Fuzz {
        /// The target URL. (e.g., http://example.com/FUZZ)
        target: String,

        /// Path to a wordlist file
        wordlist: String,

        /// HTTP request timeout length in seconds
        #[arg(short, long, default_value_t = 4)]
        timeout: u64,

        /// Number of concurrent threads
        #[arg(short, long, default_value_t = 200)]
        concurrency: usize,
    },
}
