# List just commands
default:
    @just --list

# TODO: Config git DCO and other things for contributions
setup:
    pass

# Builds all crates in repo
build:
    cargo build --workspace

# Tests all crates in repo
test:
    cargo nextest run --workspace

# Tests a specific crate in repo, provide the crate name
test-crate crate:
    cargo nextest run -p {{ crate }}

# Full testing suite
test-full:
    cargo nextest run --workspace --run-ignored all

# Linting with clippy
lint:
    RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets

# Formatting for crates and nix flake
fmt:
    cargo fmt --all
    nixfmt flake.nix

# Cargo API docs
docs:
    cargo doc --workspace --no-deps --open

# Cargo API docs test
docs-test:
    cargo doc --workspace --no-deps --document-private-items

# Mdbook serve
book:
    mdbook serve docs/

# Mdbook test
book-test:
    mdbook test docs/

# Watch test cases
watch:
    bacon test

# Watch test cases for specific crate, provide crate name
watch-crate crate:
    bacon -j test -- -p {{ crate }}

# Routine used for Github CI
ci: lint test-full

# Full Github CI precheck, run before committing to ensure that the code is in a good state
check: ci docs-test book-test
    nix flake check
    typos
    cargo doc --workspace --no-deps --document-private-items
