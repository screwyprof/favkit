# Initial Logical Architecture

## Status
accepted

## Context
We need to establish the high-level logical architecture for FavKit. The tool needs to:
- Interact with macOS Finder favorites
- Be usable both programmatically and from command line
- Support future functionality extensions
- Be maintainable and testable

## Decision
Structure the project into these logical components:

1. **Core Domain**
   - Domain models for Finder favorites
   - Core business logic and rules
   - Pure Rust implementation

2. **Infrastructure**
   - macOS system integration layer
   - Finder favorites data access
   - System-level operations

3. **Application**
   - Use case implementations
   - Coordination between domain and infrastructure
   - Error handling and data transformation

4. **Interfaces**
   - Library API for programmatic access
   - CLI for command-line usage

## Consequences

### Positive
- Clear separation of concerns
- Independent domain logic
- Flexibility in implementation details
- Easier to test each layer

### Negative
- More complex than single-layer design
- Requires careful interface design
- Higher initial development effort

### Neutral
- Need to maintain boundaries between layers
- Each layer requires its own documentation