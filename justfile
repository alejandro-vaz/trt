# show all available recipes
help:
    @just --list

# setup required dependencies for developing trt on macos
[macos]
setup:
    brew install markdownlint-cli
    brew install yamlfmt
    brew install taplo

# setup required dependencies for developing trt on linux
[linux]
setup:
    npm install -g markdownlint-cli
    go install github.com/google/yamlfmt/cmd/yamlfmt@latest
    cargo install taplo-cli --locked

# setup required dependencies for developing trt on windows
[windows]
setup:
    npm install -g markdownlint-cli
    go install github.com/google/yamlfmt/cmd/yamlfmt@latest
    cargo install taplo-cli --locked

# build and install the local development trt
install:
    cargo install --path .

# shows statistics about the current trt
stat: install
    trt version
    stat -f%z "$HOME/.cargo/bin/trt" | xargs -I{} echo "Binary size: {}B"

# format the project
format:
    cargo fmt --all
    markdownlint --fix ./README.md .github/README.md
    yamlfmt .github/workflows/*
    taplo fmt
    just --fmt
