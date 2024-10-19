use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "furl")]
#[command(author = "aliqut")]
#[command(version = "0.1.0")]
#[command(about = "Web fuzzer written in Rust")]
pub struct Cli {
    /// The target URL. (e.g., http://example.com/FUZZ)
    pub target: String,

    /// Path to a wordlist file
    pub wordlist: String,

    /// HTTP request timeout length in seconds
    #[arg(short, long, default_value_t = 4)]
    pub timeout: u64,

    /// Number of concurrent threads
    #[arg(short, long, default_value_t = 200)]
    pub concurrency: usize,

    /// Filter by HTTP status code. (Accepts a comma-separated list, e.g., "200,301,401")
    #[arg(short = 's', long)]
    pub filter_status: Option<String>,

    /// Filter by content size. (Accepts a comma-separated list, e.g., "100-200,300")
    #[arg(short = 'z', long)]
    pub filter_size: Option<String>,

    /// Filter by number of lines. (Accepts a comma-separated list, e.g., "10-20,50")
    #[arg(short = 'l', long)]
    pub filter_lines: Option<String>,
}
