default:
    @just --list

# TODO: Config git DCO and other things for contributions
setup:
    pass

build:
    cargo build --workspace

test:
    cargo nextest run --workspace

test-crate crate:
    cargo nextest run -p {{ crate }}

test-full:
    cargo nextest run --workspace --run-ignored all

lint:
    RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets

fmt:
    cargo fmt --all
    nixfmt flake.nix

api-docs:
    cargo doc --workspace --no-deps --open

api-docs-test:
    cargo doc --workspace --no-deps --document-private-items

book:
    mdbook serve docs/

book-test:
    mdbook test docs/

docs: api-docs-test book-test

watch:
    bacon test

watch-crate crate:
    bacon -j test -- -p {{ crate }}

release-all:
    cargo zigbuild

ci: lint test-full

# Run before commiting to ensure that the code is in a good state
check: ci api-docs-test book-test
    nix flake check
    typos
    cargo doc --workspace --no-deps --document-private-items
