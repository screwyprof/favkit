{
  description = "FavKit - MacOS Finder sidebar manager";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust-bin = pkgs.rust-bin;
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            (rust-bin.nightly.latest.default.override {
              extensions = [ "rust-src" "llvm-tools-preview" ];
            })
            cargo-watch
            cargo-binutils
            grcov
          ];

          buildInputs = with pkgs; [
            # MacOS specific dependencies
            darwin.apple_sdk.frameworks.CoreServices
            darwin.apple_sdk.frameworks.CoreFoundation
          ];

          # Environment variables for code coverage
          CARGO_INCREMENTAL = "0";
          RUSTFLAGS = "-Cinstrument-coverage --cfg coverage_nightly";
          LLVM_PROFILE_FILE = "target/coverage/coverage-%p-%m.profraw";

          shellHook = ''
            # Install cargo-llvm-cov if not already installed
            if ! command -v cargo-llvm-cov &> /dev/null; then
              echo "Installing cargo-llvm-cov..."
              cargo install cargo-llvm-cov
            fi
          '';
        };
      });
}
