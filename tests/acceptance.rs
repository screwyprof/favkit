use favkit::error::Result;
use favkit::finder::{Sidebar, SidebarItem};

mod test_doubles {
    use super::*;

    pub struct TestFinder {
        sidebar: Sidebar,
    }

    impl TestFinder {
        pub fn with_empty_sidebar() -> Self {
            Self {
                sidebar: Sidebar::default(),
            }
        }

        pub fn with_home_in_favorites() -> Self {
            let mut sidebar = Sidebar::default();
            sidebar.favorites_mut().add(SidebarItem::home());
            Self { sidebar }
        }

        pub fn sidebar(&self) -> &Sidebar {
            &self.sidebar
        }
    }
}

use test_doubles::TestFinder;

#[test]
fn lists_empty_favorites() -> Result<()> {
    // Given: Finder is available
    let finder = TestFinder::with_empty_sidebar();
    
    // When: We request favorites from the sidebar
    let favorites = finder.sidebar().favorites().items();
    
    // Then: The favorites section is empty
    assert!(favorites.is_empty());
    
    Ok(())
}

#[test]
fn shows_home_in_favorites() -> Result<()> {
    // Given: Finder has Home in favorites
    let finder = TestFinder::with_home_in_favorites();
    
    // When: We request favorites from the sidebar
    let favorites = finder.sidebar().favorites().items();
    
    // Then: We should see the Home item
    assert_eq!(favorites.len(), 1);
    assert_eq!(favorites[0].label(), "Home");
    
    Ok(())
}
