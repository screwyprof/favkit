# Development shell for FavKit
{ pkgs ? import <nixpkgs> { 
    overlays = [ (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz")) ];
  } 
}:

let
  toolchain = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
    extensions = [ "rust-src" "llvm-tools-preview" "rustfmt" "clippy" ];
  };

  # Development tools
  devTools = with pkgs; [
    cargo-watch
    cargo-binutils
  ];

  # macOS specific dependencies
  darwinDeps = with pkgs.darwin.apple_sdk.frameworks; [
    CoreServices
    CoreFoundation
  ];

  # Environment variables for code coverage
  coverageEnv = {
    CARGO_INCREMENTAL = "0";
    RUSTFLAGS = "-Cinstrument-coverage --cfg coverage_nightly";
    LLVM_PROFILE_FILE = "target/coverage/coverage-%p-%m.profraw";
  };
in
pkgs.mkShell {
  # Build inputs
  nativeBuildInputs = [ toolchain ] ++ devTools;
  buildInputs = darwinDeps;

  # Environment
  inherit (coverageEnv) CARGO_INCREMENTAL RUSTFLAGS LLVM_PROFILE_FILE;

  # Shell initialization
  shellHook = ''
    # Add cargo bin directory to PATH
    export PATH="$HOME/.cargo/bin:$PATH"
    
    echo "FavKit development environment loaded"
    
    # Check if cargo-llvm-cov is available in PATH after adding cargo bin
    if ! type cargo-llvm-cov >/dev/null 2>&1; then
      echo "Installing cargo-llvm-cov..."
      cargo install cargo-llvm-cov
    fi
  '';
}
