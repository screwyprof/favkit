use favkit::error::Result;
use favkit::finder::{Sidebar, SidebarItem};
use anyhow::{anyhow, Result as AnyhowResult};

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
fn shows_home_in_favorites() -> AnyhowResult<()> {
    // Given: Finder has Home in favorites
    let finder = TestFinder::with_home_in_favorites();
    
    // When: We request favorites from the sidebar
    let favorites = finder.sidebar().favorites().items();
    
    // Then: We should see the Home item with its target
    assert_eq!(favorites.len(), 1);
    let home_item = &favorites[0];
    
    assert_eq!(home_item.label(), "Home");
    assert!(home_item.path().ok_or(anyhow!("path not found"))?.ends_with("~/"));
    
    Ok(())
}
