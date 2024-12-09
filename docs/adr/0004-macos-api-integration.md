# macOS API Integration

## Status
accepted

## Context
The project requires access to macOS Finder favorites through system APIs.

## Decision
Use Rust bindings to Core Foundation and Core Services:
- [core-foundation](https://crates.io/crates/core-foundation): Core Foundation type wrappers
- [core-services](https://crates.io/crates/core-services): Core Services API access

## Consequences

### Positive
- Type-safe interface to macOS APIs
- Memory management through Rust ownership where possible

### Negative
- Requires unsafe code blocks for CF API calls
- Must follow Core Foundation reference counting rules
