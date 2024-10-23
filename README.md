# FuzzBuzz

FuzzBuzz is a high-performance, multi-threaded web fuzzer written in Rust.

It is made for penetration testers and web security researchers to find hidden files and endpoints by bruteforcing URLs.

FuzzBuzz is heavily inspired by [ffuf](https://github.com/ffuf/ffuf).

## Features

- **Concurrency**: Multiple threads for faster fuzzing.
- **Filtering**: Filter/match output results by HTTP status codes, content size, and line count.
- **Customisable HTTP requests**: Custom headers, cookies, and timeouts.
- **Redirect handling**: Follow or limit the number of HTTP redirects.

## Getting started

### Prerequisites

Make sure you have [Rust](https://www.rust-lang.org/learn/get-started) installed on your system. Verify this by running:

```
rustc --version
cargo --version
```

### Installation

1. Clone the repository

```
git clone https://github.com/aliqut/fuzzbuzz
cd fuzzbuzz
```

2. Build the project

```
cargo build --release
```

3. Install

```
cargo install --path .
```

## Usage

```
Web fuzzer written in Rust

Usage: fuzzbuzz [OPTIONS] <TARGET> <WORDLIST>

Arguments:
  <TARGET>    The target URL. (e.g., http://example.com/FUZZ)
  <WORDLIST>  Path to a wordlist file

Options:
  -c, --concurrency <CONCURRENCY>      Number of concurrent threads [default: 200]
      --match-status <MATCH_STATUS>    Match HTTP status code. (Accepts a comma-separated list, e.g., "200,301,401")
      --match-size <MATCH_SIZE>        Match content size. (Accepts a comma-separated list, e.g., "100-200,300")
      --match-lines <MATCH_LINES>      Match by number of lines. (Accepts a comma-separated list, e.g., "10-20,50")
      --filter-status <FILTER_STATUS>  Filter by HTTP status code. (Accepts a comma-separated list, e.g., "200,301,401")
      --filter-size <FILTER_SIZE>      Filter by content size. (Accepts a comma-separated list, e.g., "100-200,300")
      --filter-lines <FILTER_LINES>    Filter by number of lines. (Accepts a comma-separated list, e.g., "10-20,50")
  -t, --timeout <TIMEOUT>              HTTP request timeout length in seconds [default: 4]
      --headers <HEADERS>              HTTP request headers. (Accepts a comma-separated list, e.g., "key: value, another-key: another-value"
      --cookies <COOKIES>              HTTP request cookies. (Accepts a semicolon-separated list, e.g., "key=value;another-key=another-value"
  -r, --redirects <REDIRECTS>          Maximum HTTP redirect chain hops [default: 10]
  -d, --delay <DELAY>                  Time delay for each thread's HTTP requests [default: 0]
  -h, --help                           Print help
  -V, --version                        Print version

```

### Examples

- **Basic fuzzing**:
  
```
fuzzbuzz https://example.com path/to/wordlist.txt
```

- **Filtering by status codes** (e.g., filtering out responses with status 404):

```
fuzzbuzz --filter-status 404 https://example.com path/to/wordlist.txt
```

- **Matching by status codes** (e.g., Only showing responses with status 200 and 204):

```
fuzzbuzz --match-status 200,204 https://example.com path/to/wordlist.txt
```


- **Using custom headers**:

```
fuzzbuzz --headers "Authorization: Bearer <Token>" https://example.com path/to/wordlist.txt
```


- **Concurrency and timeout control**:

```
fuzzbuzz --concurrency 20 --timeout 10 https://example.com path/to/wordlist.txt
```

## Contributing
Pull requests are the best way to propose changes to the program.

1. Fork the repo and create your branch from `master`.
2. Make your changes.
3. If your change directly affects the program's functionality, update the documentation.
4. Issue a pull request

### Any contributions you make will be under the MIT Software License
In short, when you submit code changes, your submissions are understood to be under the same [MIT License](http://choosealicense.com/licenses/mit/) that covers the project.

### Report issues using Github's Issues tab.
I use GitHub issues to track public bugs. Report a bug by opening a new issue.

**Issue Reports** tend to have:

- A quick summary and/or background
- Steps to reproduce

## License
This project is licensed under the MIT License. See the LICENSE file for more details.
