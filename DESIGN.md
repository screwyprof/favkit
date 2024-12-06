# FavKit - MacOS Finder Sidebar Manager

## Domain Analysis

### Core Concepts

1. **Finder**
   - The main MacOS Finder application
   - Contains the sidebar which is our primary focus
   - Represents the current state of the Finder window
   - Uses repository pattern for loading state

2. **Sidebar**
   - A container for organized sections
   - Currently supports Favorites section
   - Will support Locations section in the future
   - Provides quick access to frequently used locations
   - Implements Default for empty state

3. **SidebarItem**
   - A sidebar entry that points to a specific target
   - Has a display label derived from its target
   - Provides a clean interface for sidebar management
   - Implements From<Target> for type-safe conversion
   - Validates paths during construction

4. **Target**
   - Represents a location that can be accessed via the sidebar
   - Handles path management and validation
   - Supports special system locations:
     - AirDrop: Network-based file sharing (`nwnode://domain-AirDrop`)
     - Home: User's home directory (`~/`)
   - Ensures type safety through `TryFrom` conversions
   - Validates paths against known valid targets

### Domain Model

```rust
finder
  └── Sidebar
       ├── Favorites
       │    └── [SidebarItem]
       │         └── Target (AirDrop | Home)
       └── Locations (future)
```

### Relationships

1. **Finder → Sidebar**
   - One-to-one: Each Finder has exactly one Sidebar
   - Sidebar is loaded through SidebarRepository
   - Falls back to default if loading fails

2. **Sidebar → Sections**
   - One-to-many: Each Sidebar has multiple sections
   - Currently only Favorites section is implemented
   - Each section manages its own collection of SidebarItems
   - Sections provide iterator access to their SidebarItems

3. **SidebarItem → Target**
   - One-to-one: Each SidebarItem wraps exactly one Target
   - Target determines the item's behavior and properties
   - Provides type-safe construction through From trait

## Design Decisions

### 1. Pure Domain Model
- Core domain model is completely pure with no side effects
- Separation between domain logic and MacOS API interactions
- Each component has a single responsibility
- Repository pattern for MacOS API isolation

### 2. Module Organization
- All components are organized under the `finder` module
- Clear hierarchy matching the domain model:
  ```
  src/
  ├── errors.rs      # Custom error types
  ├── lib.rs         # Public interface
  └── finder/
      ├── mod.rs         # Finder type
      ├── macos.rs       # MacOS API interface
      ├── macos_impl.rs  # MacOS API implementation
      ├── sidebar.rs     # Sidebar container
      ├── target.rs      # Target enum and conversions
      └── sidebar_item.rs # Individual items
  ```

### 3. Error Handling
- Custom error type using thiserror
- Result type alias for consistent error handling
- Three error variants:
  - InvalidPath: For path validation failures
  - UnsupportedTarget: For unsupported sidebar locations
  - Other: For wrapping external errors
- Infallible operations use unwrap in tests
- Fallible operations use ? operator for propagation

### 4. Testing Strategy
1. **Acceptance Tests**
   - Test complete user scenarios
   - Use Result return type for proper error handling
   - Mock MacOS API for system interactions
   - Verify end-to-end behavior

2. **Unit Tests**
   - Test individual components in isolation
   - Focus on error cases and edge conditions
   - Use type system to prevent invalid states

3. **Test Utilities**
   - MockMacOsApi for system interaction testing
   - Test-specific Target constructors
   - Shared test constants

### 5. MacOS Integration
- Abstract MacOS API behind traits
- Use CoreFoundation types directly:
  - CFURL for path handling
  - CFArray for collections
- Handle null pointers and optional values safely
- Mock system calls for testing

### 6. Type Safety
- Use From/TryFrom for safe conversions
- Validate paths at construction time
- Prevent invalid states through type system
- Use Option/Result for fallible operations

## Implementation Details

### 1. Core Domain Types
```rust
pub struct Finder<T: MacOsApi> {
    repo: SidebarRepository<T>
}

pub struct Sidebar {
    favorites: Vec<SidebarItem>,
    // locations: Vec<SidebarItem>, // future
}

pub struct SidebarItem {
    target: Target
}

pub enum Target {
    AirDrop(PathBuf),
    Home(PathBuf)
}
```

### 2. Current API
```rust
// Reading state
let api = SystemMacOsApi::default();
let repo = SidebarRepository::new(api);
let finder = Finder::new(repo);
let favorites = finder.sidebar().favorites();

// Creating items
let home = Target::home();
let sidebar_item = SidebarItem::from(home);
```

### 3. Error Handling
```rust
pub type Result<T> = std::result::Result<T, FinderError>;

#[derive(Debug, Error)]
pub enum FinderError {
    #[error("Invalid path: {path}")]
    InvalidPath {
        path: PathBuf,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    #[error("Unsupported target path: {0}")]
    UnsupportedTarget(PathBuf),
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}
