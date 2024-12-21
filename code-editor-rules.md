# FavKit

Every time you choose to apply a rule(s), explicitly state the rule(s) in the output. You can abbreviate the rule description to a single word or phrase.

## Project Context

A modern Rust library and CLI tool for managing macOS Finder favorites, serving as a replacement for the abandoned `mysides` tool.

- Clean Architecture implementation in Rust
- Outside-In TDD (London School) showcase
- Domain-Driven Design principles application
- Modern development practices with nix + direnv

## Code Style and Structure

- Write idiomatic, clean Rust code following community guidelines
- Use functional programming patterns where appropriate
- Prefer composition over inheritance
- Use descriptive variable names that indicate purpose
- Structure repository files as follows:

```
src/
├── finder/                 # Domain layer - pure types and high-level business logic
├── system/                 # Implementation layer
    ├── core_foundation.rs  # Safe wrappers around Core Foundation types
    ├── favorites/          # Safe wrapper around macOS API for favorites manipulation
    └── macos/              # Low-level macOS API calls implementation
tests/                      # Acceptance and integration tests
├── mock/                   # Test doubles
docs/                       # Project documentation
└── adr/                    # Architecture Decision Records
```

## Tech Stack

- Rust (nightly toolchain)
  - Edition 2024
  - Components: rustc, rust-std, rust-src, rust-analyzer, rust-docs, rustfmt, cargo, clippy, llvm-tools-preview
- [Core Foundation](https://docs.rs/core-foundation/latest/core_foundation/)
- [Core Services](https://docs.rs/core-services/latest/core_services/)
- [thiserror](https://docs.rs/thiserror/latest/thiserror/)
- [dirs](https://docs.rs/dirs/latest/dirs/)
- Development Tools:
  - [cargo-nextest](https://github.com/nextest-rs/nextest)
  - [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)
  - [bacon](https://github.com/Canop/bacon)

## Rust Usage

- Use traits for abstraction and dependency inversion
- Use proper error handling with Result and Option
- Prefer functional programming style over imperative
- Prefer method chaining over explicit loops (e.g., `iter().map().filter().collect()`)
- Prefer standard library traits (e.g., `From`, `Into`, `AsRef`) over custom traits when possible
- Use iterators and combinators over explicit loops

## Error Handling

- Use thiserror for error definitions
- Implement custom error types for each module
- Provide clear error messages
- Use Result type for fallible operations
- Implement proper error conversion traits

## Testing

- Write acceptance tests first (ATDD)
- Implement unit tests for all modules
- Use test doubles (mocks) only for the outermost layer (macos api)
- Test error cases thoroughly
- Prefer result type for fallible operations over unwrap

## State Management

- Use immutable data structures when possible

## Syntax and Formatting

- Use rustfmt for consistent formatting (make fmt)
- Follow Rust naming conventions
- Implement proper documentation comments
- Use clippy for code quality (make lint)

## Documentation

- Maintain clear README with setup instructions
- Document public APIs with examples
- Keep ADRs up to date
- Include usage examples
- Document error conditions

## Development Workflow

- Follow (A)TDD cycle
- Follow semantic versioning

## Git Usage

Follow the Conventional Commits specification at https://www.conventionalcommits.org/en/v1.0.0/

Format: `<type>[optional scope]: <description>`

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types:
- `feat:` new features
- `fix:` bug fixes
- `refactor:` code changes that neither fix bugs nor add features
- `docs:` documentation changes
- `chore:` maintenance tasks, dependency updates, etc.

Breaking changes can be indicated by adding a `!` after the type/scope or by adding a `BREAKING CHANGE:` footer.

## Security

- Handle sensitive data properly
- Implement proper error handling
- Follow Rust security best practices
- Use safe Rust by default
- Document security considerations

## Core Foundation Memory Management

We work with Core Foundation types through two layers of abstraction:

1. The [core-foundation](https://docs.rs/core-foundation/latest/core_foundation/) Rust crate, which provides safe wrappers around:
   - [CFType](https://docs.rs/core-foundation/latest/core_foundation/base/index.html) - base type
   - [CFString](https://docs.rs/core-foundation/latest/core_foundation/string/index.html) - string type
   - [CFArray](https://docs.rs/core-foundation/latest/core_foundation/array/index.html) - array type
   - [CFUrl](https://docs.rs/core-foundation/latest/core_foundation/url/index.html) - URL type

2. Our own wrapper in `system/core_foundation.rs` which provides:
   - `CFRef<T>` - Type-safe wrapper around Core Foundation types
   - `RawRef<T>` - Safe wrapper around non-null raw pointers
   - Null pointer safety checks
   - Type-safe conversions

Memory Management References:
- [Memory Management Programming Guide](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/CFMemoryMgmt.html)
- [Object Lifecycle Management](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Articles/lifecycle.html)
- [Ownership Policy](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html)
- [Copy Functions](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/CopyFunctions.html)

Rules:

Reference Counting:
- Every CF object has a reference count
- `CFRetain` increments the reference count
- `CFRelease` decrements the reference count
- When reference count reaches 0, the object is deallocated

Ownership Rules:
- Functions with these words in their names give you ownership:
  - `Create` (reference count = 1)
  - `Copy` (reference count = 1)
  - `Retain` (increments reference count)
- Functions without these words do not give you ownership:
  - `Get`
  - `Find`
  - `Register`

Memory Management:
- If you own an object, you MUST call `CFRelease` exactly once
- If you don't own an object but need to keep it:
  - Call `CFRetain` to claim ownership
  - Later call `CFRelease` when done
- Never call `CFRelease` on an object you don't own
- Never access a CF object after releasing it

Copying Objects:
- For compound objects (collections):
  - `CreateCopy` performs shallow copy (only container is copied)
  - Deep copy must be done manually if needed
- When storing CF objects:
  - Either retain them with `CFRetain` or create a copy
  - Must balance with `CFRelease` when no longer needed

# Assistant Rules

1. **Minimal Changes**: Make only the changes explicitly requested by the user. Do not perform additional refactoring, cleanup, or improvements unless specifically asked.

2. **Ask First**: If you see potential improvements beyond the requested changes, ask the user first and wait for their approval before proceeding.

3. **Stay Focused**: Keep changes focused on the specific task at hand. Don't get sidetracked by unrelated improvements or refactoring opportunities.

4. **Document Changes**: Clearly explain what changes you're making and why they're necessary for the requested task.

5. **Test Impact**: Consider and communicate how your changes might affect existing tests and functionality.

6. **Preserve Style**: Follow the existing code style and patterns unless asked to change them.

7. **Maintain Context**: Keep track of the conversation context and don't revert or modify changes that were already approved.

8. **Verify First**: Before making changes, verify that you understand the full scope of what's being requested.

9. **No Scope Creep**: If a change would require modifications beyond what was explicitly requested, ask for clarification first.

10. **Respect Structure**: Don't change project structure or move files unless specifically requested.
