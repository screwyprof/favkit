# FavKit Project Rules

## Project Context

FavKit is a modern Rust library and CLI tool for managing macOS Finder favorites, replacing the abandoned `mysides` tool.

## Architecture

1. **Clean Architecture**
   - Domain layer (`finder/`):
     - Pure business logic and types
     - Rich domain model (not anemic)
     - Type-driven development
     - Business rules encoded in types
     - Domain invariants enforced at compile time
   - Implementation layer (`system/`):
     - Technical details and infrastructure
     - Adapters for external systems
     - Implementation details

2. **Testing Strategy**
   - Outside-In TDD approach:
     1. Start with acceptance tests when possible
     2. Use integration tests with mocks for external APIs
     3. Work inward with unit tests
     4. Implement minimal code to pass
     5. Refactor without changing behavior
   - Test Coverage:
     - Unit tests for all modules
     - Mocks only for outermost layer (macOS API)
     - Thorough error case testing
     - Prefer Result over unwrap

3. **Project Structure**
   ```
   src/
   ├── finder/                 # Domain layer
   ├── system/                 # Implementation layer
       ├── core_foundation.rs  # CF wrappers
       ├── favorites/          # macOS API wrapper
       └── macos/              # Low-level API calls
   tests/                      # Tests
   ├── mock/                   # Test doubles
   docs/                       # Documentation
   └── adr/                    # ADRs
   ```

## AI Agent Rules

1. **Rule Acknowledgment**
   - State which rule(s) you're following in each response
   - Can abbreviate rule descriptions to single words/phrases

2. **Change Management**
   - Make only explicitly requested changes
   - Stay focused on the specific task
   - Follow existing patterns
   - Document changes clearly
   - Keep conversation context
   - Don't revert approved changes

3. **Communication**
   - Propose improvements after requested changes
   - Wait for user approval
   - Provide clear examples
   - Explain rationale
   - Ask for clarification when in doubt

4. **Architecture Review**
   - Ensure Clean Architecture compliance
   - Follow KISS, DRY, YAGNI, SOLID principles
   - Verify domain isolation
   - Check separation of concerns
   - Review and update ADRs

## Technical Requirements

1. **Tech Stack**
   ```toml
   [toolchain]
   rust = "nightly"
   edition = "2024"
   components = [
     "rustc", "rust-std", "rust-src",
     "rust-analyzer", "rust-docs",
     "rustfmt", "cargo", "clippy",
     "llvm-tools-preview"
   ]

   [dependencies]
   core-foundation = "*"  # https://docs.rs/core-foundation/latest/core_foundation/
   core-services = "*"    # https://docs.rs/core-services/latest/core_services/
   thiserror = "*"       # https://docs.rs/thiserror/latest/thiserror/
   dirs = "*"            # https://docs.rs/dirs/latest/dirs/

   [dev-dependencies]
   cargo-nextest = "*"   # https://github.com/nextest-rs/nextest
   cargo-llvm-cov = "*"  # https://github.com/taiki-e/cargo-llvm-cov
   bacon = "*"           # https://github.com/Canop/bacon
   ```

2. **Rust Development**
   - Type System Usage:
     - Express domain concepts through types
     - Encode business rules in type system
     - Use type-driven development
     - Prevent invalid states at compile time
     ```rust
     // WRONG: Stringly-typed API
     fn add_favorite(path: String) -> Result<(), Error>
     
     // RIGHT: Domain concepts in types
     struct Target {
         path: ValidPath,
         kind: TargetKind,
     }
     ```

   - Standard Library Traits:
     ```rust
     // WRONG: Custom conversion methods
     fn url_to_target() -> Target
     fn as_string() -> String
     fn into_bytes() -> Vec<u8>
     
     // RIGHT: Standard library traits
     impl From<Url> for Target
     impl AsRef<str> for Type
     impl Into<Vec<u8>> for Type
     ```

   - Code Organization:
     - Separate data from behavior
     - Use traits for abstraction
     - Prefer functional style over OOP
     - Use Rust idioms effectively
     - Method chaining when appropriate
     - Iterators and combinators

   - Error Handling:
     - thiserror for definitions
     - Custom error types per module
     - Result for fallible operations
     - Proper error conversion traits

3. **Documentation**
   - Clear README
   - API examples
   - Up-to-date ADRs
   - Usage examples
   - Error conditions

4. **Git Commits**
   Follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification.
   
   When the user says "let's commit" or similar, only propose a commit message following this format - do not execute any git commands.

   Format: `<type>[optional scope]: <description>`
   ```
   <type>[optional scope]: <description>

   [optional body]

   [optional footer(s)]
   ```
   Types:
   - feat: features (correlates with MINOR in semver)
   - fix: bug fixes (correlates with PATCH in semver)
   - refactor: code changes that neither fix bugs nor add features
   - docs: documentation changes
   - chore: maintenance tasks, dependency updates, etc.
   
   Breaking changes can be indicated by:
   - Adding a `!` after the type/scope: `feat!:` or `feat(api)!:`
   - Adding `BREAKING CHANGE:` in the footer

   Example with breaking change:
   ```
   feat!: drop support for Node 6

   BREAKING CHANGE: use JavaScript features not available in Node 6.
   ```

## Core Foundation Rules

1. **Overview**
   Most memory management is handled by core-foundation and core-services crates.
   Our wrapper in `system/core_foundation.rs` provides additional safety:
   - Type-safe wrappers around CF types
   - Safe handling of raw pointers
   - Proper memory management in test doubles

   Core Foundation Documentation:
   - [Memory Management Guide](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/CFMemoryMgmt.html)
   - [Object Lifecycle](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Articles/lifecycle.html)
   - [Ownership Policy](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html)
   - [Copy Functions](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/CopyFunctions.html)

2. **Core Foundation Types**
   - [CFType](https://docs.rs/core-foundation/latest/core_foundation/base/index.html) - base type
   - [CFString](https://docs.rs/core-foundation/latest/core_foundation/string/index.html) - string type
   - [CFArray](https://docs.rs/core-foundation/latest/core_foundation/array/index.html) - array type
   - [CFUrl](https://docs.rs/core-foundation/latest/core_foundation/url/index.html) - URL type

3. **Memory Management in Mocks**
   When working directly with CF types:
   - Track owned objects in mock structures
   - Balance CFRetain/CFRelease calls
   - Never access released objects
   - Be extra careful with raw pointers
   - Retain objects you need to keep
   - Release when mock is dropped
   - Never release objects you don't own
   - Prevent dangling pointers
