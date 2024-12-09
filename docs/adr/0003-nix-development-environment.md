# Nix Development Environment

## Status
accepted

## Context
Development environment setup requires:
- Rust toolchain
- macOS SDK headers
- Development tools

## Decision
Use nix flakes with shell.nix compatibility:
- Define development environment in `flake.nix`
- Provide `shell.nix` for non-flake users
- Automate environment activation with `.envrc`

## Consequences

### Positive
- Reproducible development environment
- Automated dependency management
- Compatible with both flake and legacy nix-shell workflows

### Negative
- Learning curve for nix configuration
- Additional setup step for developers
