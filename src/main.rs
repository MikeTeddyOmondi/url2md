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

fn convert_html(html: &str, no_images: bool) -> Result<String> {
    let converter = HtmlToMarkdown::builder()
        .skip_tags(if no_images {
            vec!["script", "style", "img"]
        } else {
            vec!["script", "style"]
        })
        .build();

    converter
        .convert(html)
        .map_err(|e| AppError::ConversionFailed(e.to_string()))
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

    let markdown = convert_html(&html, args.no_images)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    // ── Conversion ────────────────────────────────────────────────────────────

    #[test]
    fn converts_heading() {
        let md = convert_html("<h1>Hello</h1>", false).unwrap();
        assert!(md.contains("# Hello"), "got: {md}");
    }

    #[test]
    fn converts_paragraph() {
        let md = convert_html("<p>Hello world</p>", false).unwrap();
        assert!(md.contains("Hello world"), "got: {md}");
    }

    #[test]
    fn converts_link() {
        let md = convert_html(r#"<a href="https://example.com">click</a>"#, false).unwrap();
        assert!(md.contains("[click](https://example.com)"), "got: {md}");
    }

    #[test]
    fn strips_script_always() {
        let md = convert_html("<script>alert('x')</script><p>text</p>", false).unwrap();
        assert!(!md.contains("alert"), "got: {md}");
        assert!(md.contains("text"), "got: {md}");
    }

    #[test]
    fn strips_style_always() {
        let md = convert_html("<style>body{color:red}</style><p>text</p>", false).unwrap();
        assert!(!md.contains("color"), "got: {md}");
        assert!(md.contains("text"), "got: {md}");
    }

    #[test]
    fn keeps_images_by_default() {
        let md = convert_html(r#"<img src="photo.png" alt="photo">"#, false).unwrap();
        assert!(md.contains("photo"), "got: {md}");
    }

    #[test]
    fn strips_images_when_no_images() {
        let md = convert_html(r#"<img src="photo.png" alt="photo">"#, true).unwrap();
        assert!(!md.contains("photo"), "got: {md}");
    }

    #[test]
    fn empty_html_does_not_panic() {
        convert_html("", false).unwrap();
    }

    // ── CLI argument parsing ──────────────────────────────────────────────────

    #[test]
    fn cli_parses_url() {
        let args = Args::try_parse_from(["url2md", "https://example.com"]).unwrap();
        assert_eq!(args.url, "https://example.com");
        assert!(args.output.is_none());
        assert!(!args.no_images);
    }

    #[test]
    fn cli_parses_output_long() {
        let args =
            Args::try_parse_from(["url2md", "https://example.com", "--output", "out.md"]).unwrap();
        assert_eq!(args.output.as_deref(), Some("out.md"));
    }

    #[test]
    fn cli_parses_output_short() {
        let args = Args::try_parse_from(["url2md", "https://example.com", "-o", "out.md"]).unwrap();
        assert_eq!(args.output.as_deref(), Some("out.md"));
    }

    #[test]
    fn cli_parses_no_images() {
        let args = Args::try_parse_from(["url2md", "https://example.com", "--no-images"]).unwrap();
        assert!(args.no_images);
    }

    #[test]
    fn cli_requires_url() {
        assert!(Args::try_parse_from(["url2md"]).is_err());
    }
}
