# Project details
CLI_NAME := "gitrusty"

# Default recipe
_default:
    @just --list --unsorted

# Build project
build:
    cargo build
    @echo "Project built with Cargo"

# Run tests
test: build
    cargo test

# Create a release
release:
    #!/usr/bin/env bash
    # Colors
    RED='\033[0;31m'
    GREEN='\033[1;32m'
    NC='\033[0m'
    cargo build --release
    ls -la target/release/
    BIN_PATH="target/release/{{ CLI_NAME }}"
    if [ ! -f "$BIN_PATH" ]; then
        echo -e "${RED}Failed to build binary!${NC}"
        exit 1
    fi
    cp "${BIN_PATH}" {{ CLI_NAME }}
    echo -e "${GREEN}Created binary!${NC}"
    echo -e "${GREEN}Run the CLI using: {{ CLI_NAME }} --help${NC}"
