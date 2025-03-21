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
