_:
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

# uninstall the local development trt
uninstall:
    test -f "$HOME/.cargo/bin/trt"
    cargo uninstall trt

# shows statistics about the current trt
stat:
    test -f "$HOME/.cargo/bin/trt"
    stat -f%z "$HOME/.cargo/bin/trt" | xargs -I{} echo "Binary size: {}B"

# format the project
format:
    cargo fmt --all
    markdownlint --fix ./README.md .github/*.md LICENSE.md
    yamlfmt .github/workflows/*
    taplo fmt
    just --fmt
