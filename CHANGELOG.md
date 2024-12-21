# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0](https://github.com/screwyprof/favkit/releases/tag/v0.1.0)

### Features
- List Finder sidebar favorites with proper error handling and reporting

### Documentation
- Architecture
  - Clean Architecture implementation in Rust
  - Domain-driven design for Finder favorites
  - Core Foundation memory management patterns
- Development
  - Outside-In TDD (London School) workflow
  - Code style and structure guidelines
  - Error handling patterns
- Requirements
  - Project goals and scope
  - Supported macOS features
  - API compatibility
- ADRs
  - ADR 0001: Clean Architecture for macOS Finder favorites management
  - ADR 0002: Outside-In TDD (London School) for development approach
  - ADR 0003: Nix + direnv for reproducible development environment
  - ADR 0004: Core Foundation memory management strategy
  - ADR 0005: Error handling with custom error types
  - ADR 0006: Release Please automation for release management

### Internal
- Type-safe wrappers around Core Foundation

### Build
- Nix-based development setup with direnv
- Makefile for common development tasks
- Pre-commit hooks for code quality
- Bacon for development workflow
- Cargo tools integration (nextest, llvm-cov, rustfmt, clippy)

### CI
- Automated testing and linting workflow
- Code coverage reporting
- Release automation with Release Please
