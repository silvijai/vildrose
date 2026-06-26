{
  description = "vildrose: a ternary computer system implementation.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
            "rustfmt"
            "llvm-tools-preview"
          ];
        };

        python = pkgs.python313;
      in
      {
        devShells.default = pkgs.mkShell {
          name = "vildrose-dev";

          buildInputs = [
            # Rust
            rustToolchain

            # Python (packages managed via pyproject.toml + uv)
            python
            pkgs.uv

            # Build / task runner
            pkgs.just
            pkgs.lld

            # Cargo tools
            pkgs.cargo-nextest
            pkgs.cargo-watch
            pkgs.cargo-expand
            pkgs.cargo-deny
            pkgs.cargo-zigbuild
            pkgs.sccache

            # Background checker
            pkgs.bacon

            # Quality
            pkgs.typos

            # Documentation
            pkgs.mdbook

            # Changelog
            pkgs.git-cliff

            # Nix formatting
            pkgs.nixfmt-rfc-style

            # Utilities
            pkgs.jujutsu
            pkgs.git
            pkgs.ripgrep
            pkgs.fd
          ];

          env = {
            RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          };

          shellHook = ''
            echo ""
            echo "  vildrose dev  ($(rustc --version))"
            echo "  just          — list all recipes"
            echo ""
          '';
        };

        formatter = pkgs.nixfmt-rfc-style;
      }
    );
}
