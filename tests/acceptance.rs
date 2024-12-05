use favkit::errors::{Result, FinderError};
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
    let finder = TestFinder::with_empty_sidebar();
    let sidebar = finder.sidebar();

    assert!(sidebar.favorites().items().is_empty());
    Ok(())
}

#[test]
fn shows_home_in_favorites() -> Result<()> {
    let finder = TestFinder::with_home_in_favorites();
    let sidebar = finder.sidebar();
    let favorites = sidebar.favorites().items();
    
    assert_eq!(favorites.len(), 1, "favorites should contain one item");
    
    let home = &favorites[0];
    assert_eq!(home.label(), "Home");
    
    let path = home.path()
        .ok_or_else(|| FinderError::invalid_path("~/"))?;
    assert!(path.ends_with("~/"));

    Ok(())
}
