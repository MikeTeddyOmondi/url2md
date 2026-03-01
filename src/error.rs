use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
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
    RateLimited { url: String, retry_after_secs: u64 },

    // --- Catch-all ---
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

pub type Result<T> = std::result::Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_error_message() {
        let e = AppError::HttpError {
            url: "https://example.com".into(),
            status: reqwest::StatusCode::NOT_FOUND,
        };
        assert_eq!(
            e.to_string(),
            "HTTP error 404 Not Found when fetching 'https://example.com'"
        );
    }

    #[test]
    fn conversion_failed_message() {
        let e = AppError::ConversionFailed("bad html".into());
        assert_eq!(
            e.to_string(),
            "Failed to convert HTML to Markdown: bad html"
        );
    }

    #[test]
    fn invalid_url_message() {
        let e = AppError::InvalidUrl("not-a-url".into());
        assert_eq!(e.to_string(), "Invalid URL 'not-a-url'");
    }

    #[test]
    fn write_file_message() {
        let e = AppError::WriteFile {
            path: "/tmp/out.md".into(),
            source: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied"),
        };
        assert!(e.to_string().contains("/tmp/out.md"), "got: {e}");
        assert!(e.to_string().contains("denied"), "got: {e}");
    }
}
