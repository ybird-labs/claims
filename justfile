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

coverage:
    cargo llvm-cov --workspace --all-targets --lcov --output-path lcov.info

crap: coverage
    cargo crap --lcov lcov.info

crap-check: coverage
    cargo crap --lcov lcov.info --fail-above --threshold 30

crap-baseline: coverage
    cargo crap --lcov lcov.info --format json --output crap-baseline.json

crap-regression: coverage
    cargo crap --lcov lcov.info --baseline crap-baseline.json --fail-regression

run *args:
    cargo run {{args}}

nix-check:
    nix flake check

nix-build:
    nix build

nix-develop:
    nix develop
