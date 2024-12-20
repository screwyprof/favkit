{
  description = "FavKit - MacOS Finder sidebar manager";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      system = "aarch64-darwin";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };

      # Rust toolchain
      toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

      # Development tools
      devTools = with pkgs; [
        # Cargo extensions
        bacon
        cargo-edit
        cargo-audit
        cargo-binutils
        cargo-nextest

        # Coverage tools
        lcov
        rustfilt
      ];

      # macOS specific dependencies
      darwinDeps = with pkgs; [
        darwin.apple_sdk.frameworks.CoreServices
        darwin.apple_sdk.frameworks.CoreFoundation
      ];

      # All Rust-related environment variables
      rustEnv = {
        RUST_BACKTRACE = "1";
        CARGO_NET_GIT_FETCH_WITH_CLI = "true";
        
        # Add cargo bin to PATH
        PATH = "$HOME/.cargo/bin:${pkgs.lib.makeBinPath devTools}:$PATH";
      };

      # Build the package
      favkit = pkgs.rustPlatform.buildRustPackage {
        pname = "favkit";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
        
        buildInputs = with pkgs.darwin.apple_sdk.frameworks; [
          CoreServices
          CoreFoundation
        ];

        # Use nightly toolchain
        RUSTC_BOOTSTRAP = 1;
        nativeBuildInputs = [ toolchain ];
      };
    in
    {
      # Development environment
      devShells.${system}.default = pkgs.mkShell {
        # Build inputs
        nativeBuildInputs = [ toolchain ] ++ devTools;
        buildInputs = darwinDeps;

        # Environment
        inherit (rustEnv) RUST_BACKTRACE CARGO_NET_GIT_FETCH_WITH_CLI PATH; 

        # Shell initialization 
        shellHook = ''
          echo "Rust development environment loaded"
          echo "Rust version: $(rustc --version)"
          echo "Cargo version: $(cargo --version)"

          # Check if cargo-llvm-cov is available in PATH after adding cargo bin
          if ! type cargo-llvm-cov >/dev/null 2>&1; then
            echo "Installing cargo-llvm-cov..."
            cargo install cargo-llvm-cov
          fi
        '';
      };

      # Package output
      packages.${system} = {
        default = favkit;
        favkit = favkit;
      };

      # Meta information
      meta = {
        maintainers = ["Maksim Shcherbo <max@happygopher.nl>"];
        platforms = ["x86_64-darwin" "aarch64-darwin"];
      };
    };
}
