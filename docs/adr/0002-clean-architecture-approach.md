# Use Clean Architecture with Outside-In TDD

## Status
accepted

## Context
We need to choose an architectural approach for FavKit that will:
- Support maintainable and testable code
- Handle complex macOS API integration
- Allow for iterative development
- Serve as a learning exercise

## Decision
We will use Clean Architecture with Outside-In TDD (London School) approach:
- Start with infrastructure layer (macOS API wrapper)
- Move inward to domain layer
- Follow SOLID principles
- Use DDD tactical patterns where appropriate

## Consequences

### Positive
- Clear separation of concerns
- Easier to test complex macOS APIs first
- Flexible for future changes
- Domain model emerges from real use cases

### Negative
- More initial setup required
- Might be overengineered for current scope
