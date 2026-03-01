# url2md

A fast CLI tool that fetches a URL and converts its HTML content to Markdown.
Built with [Clap](https://docs.rs/clap), [Reqwest](https://docs.rs/reqwest), and [htmd](https://docs.rs/htmd).

## Installation

### From crates.io

```bash
cargo install url2md
```

### From Docker Hub

```bash
docker pull docker.io/locci/url2md
```

### Build from source

```bash
git clone https://github.com/locci-cloud/url2md
cd url2md
cargo build --release
# binary at ./target/release/url2md
```

---

## Usage

```
url2md [OPTIONS] <URL>
```

### Options

| Flag | Short | Description |
|------|-------|-------------|
| `--output <FILE>` | `-o` | Write output to a file instead of stdout |
| `--no-images` | | Strip `<img>` tags from the converted Markdown |
| `--help` | `-h` | Print help |
| `--version` | `-V` | Print version |

### Examples

```bash
# Print Markdown to stdout
url2md https://example.com

# Save to a file
url2md https://example.com --output result.md

# Strip images
url2md https://example.com --no-images

# Combine flags
url2md https://example.com --no-images -o result.md
```

---

## Development

Install [just](https://just.systems) first:

```bash
cargo install just
```

Then use it to run common tasks:

```bash
just fmt          # apply formatting
just fmt-check    # check formatting (what CI runs)
just clippy       # lint with clippy (warnings = errors)
just test         # run tests
just ci           # full local CI pass: fmt-check → clippy → test

just build        # dev build
just build-release  # release build

just run https://example.com              # fetch and print to stdout
just save https://example.com out.md     # fetch and save to file
```

---

## Docker Usage

Run the binary directly via Docker — no Rust toolchain required.

```bash
# Print Markdown to stdout
docker run --rm docker.io/locci/url2md https://example.com

# Save output to a local file (mount current directory)
docker run --rm -v "$(pwd):/out" docker.io/locci/url2md \
  https://example.com --output /out/result.md

# Strip images
docker run --rm docker.io/locci/url2md https://example.com --no-images
```

---

## Notes

- **JS-heavy SPAs:** Reqwest fetches raw HTML only. Pages that render content
  via JavaScript (React, Vue, etc.) won't convert well. Use a headless browser
  like `chromiumoxide` for those cases.

- **Bot blocking (403s):** Some sites block scrapers. If you receive a `403`,
  the site may require a browser-like User-Agent. This is a known limitation of
  the current HTTP client.

- **TLS:** Reqwest uses [rustls](https://docs.rs/rustls) with
  [aws-lc-rs](https://docs.rs/aws-lc-rs) as the crypto backend — no system
  OpenSSL dependency required.

- **Async:** The blocking Reqwest client is used to keep the binary simple and
  dependency-light. Async support (tokio) can be added if needed.

---

## License

[MIT](./LICENSE)
