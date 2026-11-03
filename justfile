_:
    @just --list
install:
    @cargo install --path .
    @stat -f%z "$HOME/.cargo/bin/trt" | xargs -I{} echo "Binary size: {}B"
setup target:
    #!/bin/bash
    set -euo pipefail
    case "{{ target }}" in
        macos)
            brew install markdownlint-cli
            brew install yamlfmt
            brew install taplo
            ;;
        *)
            echo "Error: Unsupported target '{{ target }}'"
            just
            exit 1
            ;;
    esac
format:
    @cargo fmt --all
    @markdownlint --fix ./README.md .github/README.md
    @yamlfmt .github/workflows/*
    @taplo fmt
    @just --fmt
