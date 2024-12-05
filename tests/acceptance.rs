use favkit::errors::Result;
use favkit::finder::{Sidebar, SidebarItem, Target};

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

        pub fn with_item(mut self, item: SidebarItem) -> Self {
            self.sidebar_mut().favorites_mut().add(item);
            self
        }

        pub fn with_home(self) -> Self {
            self.with_item(SidebarItem::home())
        }

        pub fn with_airdrop(self) -> Self {
            self.with_item(SidebarItem::airdrop())
        }
    }

    pub struct FinderAssert<'a> {
        finder: &'a TestFinder,
    }

    impl<'a> FinderAssert<'a> {
        pub fn new(finder: &'a TestFinder) -> Self {
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

use test_doubles::{TestFinder, FinderAssert};

#[test]
fn shows_airdrop_in_favorites() -> Result<()> {
    let finder = TestFinder::default().with_airdrop();
    FinderAssert::new(&finder).has_favorites(&[Target::airdrop()]);
    Ok(())
}

#[test]
fn shows_home_in_favorites() -> Result<()> {
    let finder = TestFinder::default().with_home();
    FinderAssert::new(&finder).has_favorites(&[Target::home()]);
    Ok(())
}

#[test]
fn shows_empty_favorites() -> Result<()> {
    let finder = TestFinder::default();
    FinderAssert::new(&finder).is_empty();
    Ok(())
}

#[test]
fn shows_multiple_items_in_favorites() -> Result<()> {
    let finder = TestFinder::default()
        .with_home()
        .with_airdrop();
    
    FinderAssert::new(&finder).has_favorites(&[
        Target::home(),
        Target::airdrop(),
    ]);
    Ok(())
}
