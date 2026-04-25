set dotenv-load := true

default: ci

fmt:
    cargo fmt --all -- --check

check:
    cargo check --workspace --all-targets

test:
    cargo test --workspace --all-targets

clippy:
    cargo clippy --workspace --all-targets -- --deny warnings

ci: fmt check clippy test

run *args:
    cargo run {{args}}

nix-check:
    nix flake check

nix-build:
    nix build

nix-develop:
    nix develop
