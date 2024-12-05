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
   - Has a display label
   - Can point to either special system locations (AirDrop, Recents) or filesystem directories

### Domain Model

```rust
finder
  └── Sidebar
       └── Favorites
            └── [SidebarItems]
```

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
1. **Unit Tests**
   - Each component has its own tests
   - Focus on behavior verification
   - Default implementations for easy testing

2. **Acceptance Tests**
   - Test doubles (TestFinder) to simulate real Finder
   - Focus on user-facing behavior
   - Follow Given/When/Then pattern
