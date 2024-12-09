# Error Handling with thiserror

## Status
accepted

## Context
The project needs to handle errors from:
- Core Foundation API calls
- File system operations
- Invalid user input

## Decision
Use [thiserror](https://crates.io/crates/thiserror) for implementing error types:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FinderError {
    #[error("failed to access Finder favorites: {0}")]
    AccessError(#[from] core_foundation::Error),
    
    #[error("invalid favorite item: {0}")]
    InvalidItem(String),
}

pub type Result<T> = std::result::Result<T, FinderError>;
```

## Consequences

### Positive
- Compile-time error type checking
- Automatic error conversion via From trait
- Structured error messages
- Simplified type signatures

### Negative
- Additional dependency in the project
- May encourage overly complex error hierarchies
