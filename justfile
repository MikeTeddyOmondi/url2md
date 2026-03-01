set shell := ["zsh", "-cu"]

image := "docker.io/locci/url2md"

# list available recipes
default:
    @just --list

# format code
fmt:
    cargo fmt --all

# check formatting without modifying files
fmt-check:
    cargo fmt --all --check

# run clippy (warnings = errors)
clippy:
    cargo clippy --all-targets --locked -- -D warnings

# run tests
test:
    cargo test --locked

# fmt → clippy → test (mirrors CI)
ci: fmt-check clippy test

# build dev
build:
    cargo build

# build release
build-release:
    cargo build --release

# run with a URL  (usage: just run https://example.com)
run URL:
    cargo run -- "{{URL}}"

# run with a URL and save to file  (usage: just save https://example.com out.md)
save URL OUT:
    cargo run -- "{{URL}}" -o "{{OUT}}"

# ── Docker ────────────────────────────────────────────────────────────────────

# build the Docker image locally
docker-build:
    docker build -t {{image}} .

# run via Docker  (usage: just docker-run https://example.com)
docker-run URL:
    docker run --rm {{image}} "{{URL}}"

# run via Docker and save to a local file  (usage: just docker-save https://example.com out.md)
docker-save URL OUT:
    docker run --rm -v "$(pwd):/out" {{image}} "{{URL}}" -o "/out/{{OUT}}"

# pull the latest image from Docker Hub
docker-pull:
    docker pull {{image}}

# push the local image to Docker Hub
docker-push:
    docker push {{image}}
