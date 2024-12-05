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

        nativeBuildInputs = with pkgs; [
          (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          (rust-bin.stable.latest.default.override {
            extensions = [ "llvm-tools-preview" ];
          })
          cargo-watch
          grcov
        ];

        buildInputs = with pkgs; [
          # MacOS specific dependencies
          darwin.apple_sdk.frameworks.CoreServices
          darwin.apple_sdk.frameworks.CoreFoundation
        ];

      in
      {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;

          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          
          # Environment variables for code coverage
          CARGO_INCREMENTAL = "0";
          RUSTFLAGS = "-Cinstrument-coverage";
          LLVM_PROFILE_FILE = "target/coverage/coverage-%p-%m.profraw";
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "favkit";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          inherit nativeBuildInputs buildInputs;
        };
      }
    );
}
