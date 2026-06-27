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

docs:
    cargo doc --workspace --no-deps --open

book:
    mdbook serve docs/

watch:
    bacon test

watch-crate crate:
    bacon -j test -- -p {{ crate }}

release-all:
    cargo zigbuild

ci: lint test-full
    nix flake check
