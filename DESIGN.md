# FavKit - MacOS Finder Sidebar Manager

## Domain Analysis

### Core Concepts

1. **Finder**
   - The main MacOS Finder application
   - Contains the sidebar which is our primary focus
   - Represents the current state of the Finder window

2. **Sidebar**
   - A container for organized sections
   - Provides quick access to frequently used locations
   - Contains two main sections: Favorites and Locations

3. **Item**
   - A sidebar entry that points to a specific target
   - Has a display label
   - Can point to either special system locations (AirDrop, Recents) or filesystem directories

4. **Target**
   - What an item points to
   - Can be:
     - Special system targets (AirDrop, Recents, Applications)
     - Regular filesystem directories

### Domain Model

```rust
Finder
  └── Sidebar
       ├── Favorites
       │    └── [Items]
       └── Locations
            └── [Items]
```

## Design Decisions

### 1. Pure Domain Model
- Core domain model is completely pure with no side effects
- Separation between domain logic and MacOS API interactions
- Use of value objects for important concepts (Label, DirectoryPath)

### 2. Fluent API Design
- Natural method chaining that reads like English
- Section-based operations: `sidebar.favorites().add(item)`
- Intuitive querying: `sidebar.favorites().items()`

### 3. Error Handling
- Domain-specific error types
- Clear separation between domain errors and system errors
- Type-safe operations

### 4. MacOS Integration
- Gateway pattern for MacOS API interaction
- Clear separation between domain operations and system calls
- Asynchronous operations where appropriate

## Implementation Strategy

### 1. Core Domain Types
```rust
pub struct Item {
    label: Label,
    target: Target,
}

pub enum Target {
    AirDrop,
    Recents,
    Applications,
    Directory(DirectoryPath),
}

pub struct Sidebar {
    favorites: Favorites,
    locations: Locations,
}
```

### 2. API Design
```rust
// Reading state
let finder = MacOSFinder::inspect()?;
let favorites = finder.sidebar().favorites().items();

// Modifying state
finder
    .sidebar_mut()
    .favorites()
    .add(Item::airdrop());

// Applying changes
MacOSFinder::apply(&finder)?;
```

### 3. Error Handling
```rust
pub enum DomainError {
    InvalidLabel(String),
    InvalidPath(String),
    ItemNotFound(String),
    // ... other domain-specific errors
}

pub enum SystemError {
    ApiError(String),
    PermissionDenied,
    // ... system-specific errors
}
```

## Usage Examples

### Basic Operations
```rust
// Inspect current state
let finder = MacOSFinder::inspect()?;

// Add items to favorites
finder
    .sidebar_mut()
    .favorites()
    .add(Item::airdrop());

// Add directory to locations
finder
    .sidebar_mut()
    .locations()
    .add(Item::directory("Projects", "/Users/me/Projects")?);

// Apply changes
MacOSFinder::apply(&finder)?;
```

### Querying
```rust
let finder = MacOSFinder::inspect()?;
let favorite_items = finder.sidebar().favorites().items();
let location_items = finder.sidebar().locations().items();
```

## Development Approach: ATDD/TDD

### ATDD (Acceptance Test Driven Development)
1. **Write Acceptance Test First**
   - Start with a failing acceptance test that describes the feature from user's perspective
   - Test should be written in domain language
   - Test should focus on business value, not implementation details

2. **Red-Green-Refactor with TDD**
   - Write failing unit test
   - Write minimal code to make it pass
   - Refactor while keeping tests green
   - Repeat until acceptance test passes

### First Iteration
For our first iteration, we'll focus on the minimal valuable feature:
- List items from the Favorites section of the Finder sidebar

Example Acceptance Test:
```rust
#[test]
fn should_list_items_from_favorites_section() {
    // Given: A finder with items in favorites
    let finder = MacOSFinder::inspect().unwrap();
    
    // When: We request favorites items
    let items = finder.sidebar().favorites().items();
    
    // Then: We should see the actual items from MacOS Finder
    assert!(!items.is_empty());
    // Further assertions about items structure and content
}
```

This will drive our development through several TDD cycles:
1. Create domain types (Item, Target, etc.)
2. Implement Finder and Sidebar structures
3. Create MacOS gateway interface
4. Implement actual MacOS integration

## Future Considerations

1. **Additional Sections**
   - Support for iCloud section
   - Support for Tags section
   - Custom sections support

2. **Extended Functionality**
   - Reordering items within sections
   - Batch operations
   - Event notifications for sidebar changes

3. **Performance Optimizations**
   - Caching of sidebar state
   - Lazy loading of section contents
   - Batch updates to MacOS

## Testing Strategy

1. **Unit Tests**
   - Pure domain model testing
   - Value object validation
   - Error cases

2. **Integration Tests**
   - MacOS API interaction
   - System state changes
   - Error handling

3. **Property-Based Tests**
   - Item validation
   - State transitions
   - Invariant checking
