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
   - Contains the Favorites section (will add Locations later)

3. **SidebarItem**
   - A sidebar entry that points to a specific target
   - Has a display label derived from its target
   - Provides a clean interface for sidebar management

4. **Target**
   - Represents a location that can be accessed via the sidebar
   - Handles path management and validation
   - Supports special system locations:
     - AirDrop: Network-based file sharing
     - Home: User's home directory
   - Ensures type safety through `TryFrom` conversions

### Domain Model

```rust
finder
  └── Sidebar
       └── Favorites
            └── [SidebarItem]
                 └── Target (AirDrop | Home)
```

### Relationships

1. **Finder → Sidebar**
   - One-to-one: Each Finder has exactly one Sidebar
   - Sidebar is the main point of interaction

2. **Sidebar → Favorites**
   - One-to-one: Each Sidebar has one Favorites section
   - Favorites manages a collection of items

3. **SidebarItem → Target**
   - One-to-one: Each SidebarItem wraps exactly one Target
   - Target determines the item's behavior and properties
   - Provides type-safe construction through various conversions

## Design Decisions

### 1. Pure Domain Model
- Core domain model is completely pure with no side effects
- Separation between domain logic and MacOS API interactions
- Each component has a single responsibility (Finder manages Sidebar, Sidebar manages Favorites, etc.)

### 2. Module Organization
- All components are organized under the `finder` module
- Clear hierarchy matching the domain model:
  ```
  src/
  ├── error.rs
  └── finder/
      ├── mod.rs         # Finder type
      ├── sidebar.rs     # Sidebar container
      ├── favorites.rs   # Favorites section
      └── sidebar_item.rs # Individual items
  ```

### 3. Fluent API Design
- Natural method chaining that reads like English
- Intuitive querying: `finder.sidebar().favorites().items()`
- Each level returns references to allow inspection without taking ownership

### 4. Error Handling
- Minimal error handling to start
- Will expand based on actual error cases encountered
- Type-safe operations using Result

## Implementation Details

### 1. Core Domain Types
```rust
pub struct Finder {
    sidebar: Sidebar,
}

pub struct Sidebar {
    favorites: Favorites,
}

pub struct Favorites {
    items: Vec<SidebarItem>,
}

pub struct SidebarItem {
    label: String,
}
```

### 2. Current API
```rust
// Reading state
let finder = Finder::default();
let favorites = finder.sidebar().favorites().items();

// Creating items
let item = SidebarItem::airdrop();
```

### 3. Testing Strategy
1. **Domain-Driven Testing**
   - Tests mirror the domain hierarchy
   - Each domain concept is tested in isolation
   - Focus on behavior verification over implementation details

2. **Test Boundaries**
   - Clear separation between domain logic and system interactions
   - Filesystem operations are abstracted for testing
   - MacOS-specific functionality is isolated behind interfaces

3. **Test Categories**
   - Unit tests verify domain model integrity
   - Integration tests verify component interactions
   - Acceptance tests validate user scenarios

4. **Test Data**
   - Use real-world examples from MacOS Finder
   - Test data reflects actual user scenarios
   - Special locations (AirDrop, Home) are treated as first-class concepts
