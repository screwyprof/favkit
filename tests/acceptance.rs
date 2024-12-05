use favkit::errors::Result;
use favkit::finder::{Finder, Sidebar, SidebarItem, Target};

mod test_doubles {
    use super::*;

    pub struct FinderAssert<'a> {
        finder: &'a Finder,
    }

    impl<'a> FinderAssert<'a> {
        pub fn new(finder: &'a Finder) -> Self {
            Self { finder }
        }

        pub fn has_favorites(&self, expected_targets: &[Target]) -> &Self {
            let items = self.finder.sidebar().favorites().items();
            assert_eq!(items.len(), expected_targets.len(), 
                "Expected {} items in favorites, got {}", expected_targets.len(), items.len());

            for (item, expected) in items.iter().zip(expected_targets) {
                assert_eq!(item.label(), expected.label(), 
                    "Expected item to have label '{}'", expected.label());
                assert_eq!(item.path(), Some(expected.path()), 
                    "Expected item '{}' to have path '{}'", 
                    expected.label(), 
                    expected.path().display());
            }
            self
        }

        pub fn is_empty(&self) -> &Self {
            self.has_favorites(&[])
        }
    }
}

use test_doubles::FinderAssert;

#[test]
fn shows_airdrop_in_favorites() -> Result<()> {
    let mut finder = Finder::default();
    finder.sidebar_mut().favorites_mut().add(SidebarItem::airdrop());
    
    FinderAssert::new(&finder).has_favorites(&[Target::airdrop()]);
    Ok(())
}

#[test]
fn shows_home_in_favorites() -> Result<()> {
    let mut finder = Finder::default();
    finder.sidebar_mut().favorites_mut().add(SidebarItem::home());
    
    FinderAssert::new(&finder).has_favorites(&[Target::home()]);
    Ok(())
}

#[test]
fn shows_empty_favorites() -> Result<()> {
    let finder = Finder::default();
    FinderAssert::new(&finder).is_empty();
    Ok(())
}

#[test]
fn shows_multiple_items_in_favorites() -> Result<()> {
    let mut finder = Finder::default();
    finder.sidebar_mut().favorites_mut().add(SidebarItem::home());
    finder.sidebar_mut().favorites_mut().add(SidebarItem::airdrop());
    
    FinderAssert::new(&finder).has_favorites(&[
        Target::home(),
        Target::airdrop(),
    ]);
    Ok(())
}
