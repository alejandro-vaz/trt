# show all available recipes
help:
    @just --list

# setup required dependencies for developing trt on macos
[macos]
setup:
    brew install markdownlint-cli
    brew install yamlfmt
    brew install taplo

# build and install the local development trt
install:
    cargo clean
    cargo install --path .

# shows statistics about the current trt
stat:
    trt version
    stat -f%z "$HOME/.cargo/bin/trt" | xargs -I{} echo "Binary size: {}B"

# format the project
format:
    cargo fmt --all
    markdownlint --fix ./README.md .github/*.md LICENSE.md
    yamlfmt .github/workflows/*
    taplo fmt
    just --fmt
