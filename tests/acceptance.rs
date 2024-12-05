use favkit::errors::Result;
use favkit::finder::{Sidebar, SidebarItem};

mod test_doubles {
    use super::*;

    pub struct TestFinder {
        sidebar: Sidebar,
    }

    impl TestFinder {
        pub fn default() -> Self {
            Self {
                sidebar: Sidebar::default(),
            }
        }

        pub fn sidebar(&self) -> &Sidebar {
            &self.sidebar
        }

        pub fn sidebar_mut(&mut self) -> &mut Sidebar {
            &mut self.sidebar
        }
    }
}

use test_doubles::TestFinder;

#[test]
fn shows_airdrop_in_favorites() -> Result<()> {
    let mut finder = TestFinder::default();
    let airdrop = SidebarItem::airdrop();
    finder.sidebar_mut().favorites_mut().add(airdrop);
    
    let favorites = finder.sidebar().favorites();
    let items = favorites.items();
    
    assert_eq!(items.len(), 1);
    let airdrop_item = &items[0];
    assert_eq!(airdrop_item.label(), "AirDrop");

    Ok(())
}

#[test]
fn shows_home_in_favorites() -> Result<()> {
    let mut finder = TestFinder::default();
    let home = SidebarItem::home();
    finder.sidebar_mut().favorites_mut().add(home);
    
    let favorites = finder.sidebar().favorites();
    let items = favorites.items();
    
    assert_eq!(items.len(), 1);
    let home_item = &items[0];
    assert_eq!(home_item.label(), "Home");

    Ok(())
}
