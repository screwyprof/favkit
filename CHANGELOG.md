# Changelog

All notable changes to this project will be documented in this file.

## 0.1.0 (2024-12-21)


### Features

* add git hooks with pre-commit checks ([1a3f935](https://github.com/screwyprof/favkit/commit/1a3f93521195e06fd2edd664451fe70a6d325462))
* add LSSharedFileListCopySnapshot to MacOsApi ([bbb5801](https://github.com/screwyprof/favkit/commit/bbb5801edac887e5fe65cec93e789877fd56f1cf))
* add Makefile linting and improve organization ([c08dcfd](https://github.com/screwyprof/favkit/commit/c08dcfdf738bbe0b5f3b8b0531461cb3b8e68169))
* add minimal FinderApi with acceptance test ([486e0a9](https://github.com/screwyprof/favkit/commit/486e0a92a7210fbc14de14f0332f1aac45de1ffe))
* add nix-filter and improve build targets ([3e22dfe](https://github.com/screwyprof/favkit/commit/3e22dfe1681f6a6687fcefa0ec69bd54e9a044fa))
* add security audit to lint target ([0aeb152](https://github.com/screwyprof/favkit/commit/0aeb1526aa6415ad0a6f5eecbcfdb83c9e5773fb))
* **favorites:** add URL resolution for sidebar items ([c5af86d](https://github.com/screwyprof/favkit/commit/c5af86dff69e2d0e1583a40e809135af068f61b0))
* implement display name retrieval for favorites ([6e28a64](https://github.com/screwyprof/favkit/commit/6e28a6498b2e35bcb70009055c8f1a17702393bd))
* introduce anti-corruption layer for favorites ([88725da](https://github.com/screwyprof/favkit/commit/88725dae859ea6c7d6aaf4f75ed3c598b067877a))
* introduce MacOS API abstraction ([31ccccd](https://github.com/screwyprof/favkit/commit/31ccccd0a4925a0774356c71498481c2631f63ad))
* migrate to cargo-nextest ([020da32](https://github.com/screwyprof/favkit/commit/020da3222ddbfbd89e9f08e6dc37e29f61728059))
* **sidebar:** implement Display trait for SidebarItem ([c211697](https://github.com/screwyprof/favkit/commit/c2116979721edeca1224682773f5312731509658))


### Bug Fixes

* **dev:** handle cargo-llvm-cov installation for non-linux platforms ([b691938](https://github.com/screwyprof/favkit/commit/b691938b172828339483acb1632a15defb2a92f5))
* treat empty display names as None in sidebar items ([108fb51](https://github.com/screwyprof/favkit/commit/108fb518f2ce1bfbb15611f699081f74eef8876d))

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
