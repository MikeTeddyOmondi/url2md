# url2md — Rust CLI Reference

A CLI tool that fetches a URL and converts its HTML content to Markdown, built with Clap (derive), Reqwest, and htmd.

---

## Project Setup

**`Cargo.toml`**

```toml
[package]
name = "url2md"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", features = ["derive"] }
reqwest = { version = "0.12", features = ["blocking"] }
htmd = "0.1"
thiserror = "1"
```

---

## Error Handling

**`src/error.rs`**

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // --- Network ---
    #[error("Failed to fetch URL '{url}': {source}")]
    FetchFailed {
        url: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("HTTP error {status} when fetching '{url}'")]
    HttpError {
        url: String,
        status: reqwest::StatusCode,
    },

    #[error("Request timed out for URL '{url}'")]
    Timeout { url: String },

    #[error("Invalid URL '{0}'")]
    InvalidUrl(String),

    // --- Conversion ---
    #[error("Failed to convert HTML to Markdown: {0}")]
    ConversionFailed(String),

    #[error("Empty or unparseable HTML content from '{url}'")]
    EmptyContent { url: String },

    // --- I/O ---
    #[error("Failed to write to file '{path}': {source}")]
    WriteFile {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to read file '{path}': {source}")]
    ReadFile {
        path: String,
        #[source]
        source: std::io::Error,
    },

    // --- Config / Args ---
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Configuration error: {0}")]
    Config(String),

    // --- Auth (future) ---
    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("API key missing or invalid")]
    MissingApiKey,

    // --- Rate limiting (future) ---
    #[error("Rate limited by '{url}', retry after {retry_after_secs}s")]
    RateLimited {
        url: String,
        retry_after_secs: u64,
    },

    // --- Catch-all ---
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
```

**Design notes:**

- Named fields on variants carry context (url, path, status) so errors are self-describing in logs.
- `#[source]` on wrapped errors preserves the error chain for debugging with `tracing` or `eyre` later.
- `pub type Result<T>` alias keeps call sites clean — just `Result<T>` everywhere.
- Grouped by domain (network, I/O, config, auth) so it's obvious where to add new variants.

---

## Main Entry Point

**`src/main.rs`**

```rust
mod error;
use error::{AppError, Result};

use clap::Parser;
use htmd::HtmlToMarkdown;

/// Convert a URL's content to Markdown
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The URL to fetch and convert
    url: String,

    /// Output file (prints to stdout if not specified)
    #[arg(short, long)]
    output: Option<String>,

    /// Strip images from output
    #[arg(long, default_value_t = false)]
    no_images: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Fetch HTML
    let response = reqwest::blocking::get(&args.url).map_err(|e| AppError::FetchFailed {
        url: args.url.clone(),
        source: e,
    })?;

    if !response.status().is_success() {
        return Err(AppError::HttpError {
            url: args.url.clone(),
            status: response.status(),
        });
    }

    let html = response.text().map_err(|e| AppError::FetchFailed {
        url: args.url.clone(),
        source: e,
    })?;

    // Convert to Markdown
    let converter = HtmlToMarkdown::builder()
        .skip_tags(if args.no_images {
            vec!["script", "style", "img"]
        } else {
            vec!["script", "style"]
        })
        .build();

    let markdown = converter
        .convert(&html)
        .map_err(|e| AppError::ConversionFailed(e.to_string()))?;

    // Output
    match args.output {
        Some(ref path) => {
            std::fs::write(path, markdown).map_err(|e| AppError::WriteFile {
                path: path.clone(),
                source: e,
            })?;
        }
        None => print!("{}", markdown),
    }

    Ok(())
}
```

---

## Usage

```bash
# Print converted markdown to stdout
cargo run -- https://example.com

# Save to a file
cargo run -- https://example.com --output output.md

# Strip images from output
cargo run -- https://example.com --no-images

# Combine flags
cargo run -- https://example.com --no-images -o output.md

# After installing with `cargo install --path .`
url2md https://example.com -o result.md
```

---

## Notes & Caveats

- **User-Agent:** Some sites block scrapers. Add `.header("User-Agent", "Mozilla/5.0 ...")` on the request builder if you get 403s.
- **JS-heavy SPAs:** Reqwest only fetches raw HTML. Pages that render content via JavaScript won't convert well — you'd need a headless browser like `chromiumoxide` for those.
- **Blocking vs Async:** The blocking Reqwest client keeps things simple. To go async, swap to `tokio` + async reqwest.
- **Error extension:** Add new variants to `AppError` grouped by domain. Use named struct variants for errors that carry context, and tuple variants for simple string messages.