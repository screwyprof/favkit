use favkit::error::Result;
use favkit::sidebar::Sidebar;

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
