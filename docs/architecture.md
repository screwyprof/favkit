# FavKit Architecture

## Overview
FavKit is designed as a modular system for viewing macOS Finder favorites, consisting of a core library and a CLI interface.

## Core Components

### Library (`src/lib.rs`)
- **FavKit**: Main library interface for accessing Finder favorites
- **Error Handling**: Custom error types via thiserror
- **macOS Integration**: Core Foundation and Core Services bindings for reading favorites

### CLI (`src/main.rs`)
- Command-line interface for viewing favorites
- User-friendly output format
- Clear error reporting

## Design Principles
1. **Clean Architecture**
   - Clear separation of concerns
   - Domain-driven design
   - SOLID principles
2. **Error Handling**
   - Comprehensive error types
   - Exhaustive error variants
3. **Testing**
   - Outside-In TDD approach
   - Integration tests for macOS API layer
   - Unit tests for domain logic
4. **Documentation**
   - Clear, concise, and maintained

## Technical Stack
- **Language**: Rust (2024 edition)
- **Build**: Nix + direnv (via flake.nix)
- **Testing**: cargo test + llvm-cov
- **macOS Integration**: core-foundation + core-services

## Development Workflow
1. Write failing test
2. Implement minimal code
3. Refactor if needed
4. Document changes
5. Update ADRs if needed
