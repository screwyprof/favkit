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
        };
      in
      {
        # Development environment
        devShells.default = import ./shell.nix { inherit pkgs; };

        # Package output
        packages = {
          default = favkit;
          favkit = favkit;
        };

        # Meta information
        meta = {
          maintainers = ["Maksim Shcherbo <max@happygopher.nl>"];
          platforms = ["x86_64-darwin" "aarch64-darwin"];
        };
      }
    );
}
