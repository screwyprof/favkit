use favkit::errors::Result;
use favkit::finder::{Finder, Sidebar, SidebarItem, Target, SidebarRepository};
use std::rc::Rc;
use std::cell::RefCell;

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

    #[derive(Clone)]
    pub struct TestSidebarRepository {
        sidebar: Rc<RefCell<Option<Sidebar>>>,
    }

    impl TestSidebarRepository {
        pub fn new() -> Self {
            Self {
                sidebar: Rc::new(RefCell::new(None)),
            }
        }
    }

    impl SidebarRepository for TestSidebarRepository {
        fn load(&self) -> Result<Sidebar> {
            Ok(self.sidebar.borrow().clone().unwrap_or_default())
        }

        fn save(&self, sidebar: &Sidebar) -> Result<()> {
            *self.sidebar.borrow_mut() = Some(sidebar.clone());
            Ok(())
        }
    }
}

use test_doubles::{FinderAssert, TestSidebarRepository};

#[test]
fn shows_airdrop_in_favorites() -> Result<()> {
    // Given AirDrop is in favorites
    let repository = TestSidebarRepository::new();
    let mut sidebar = Sidebar::default();
    sidebar.favorites_mut().add(SidebarItem::airdrop());
    repository.save(&sidebar)?;
    
    // When I start Finder
    let finder = Finder::start(Box::new(repository))?;
    
    // Then I should see AirDrop in favorites
    FinderAssert::new(&finder).has_favorites(&[Target::airdrop()]);
    Ok(())
}

#[test]
fn shows_home_in_favorites() -> Result<()> {
    // Given Home is in favorites
    let repository = TestSidebarRepository::new();
    let mut sidebar = Sidebar::default();
    sidebar.favorites_mut().add(SidebarItem::home());
    repository.save(&sidebar)?;
    
    // When I start Finder
    let finder = Finder::start(Box::new(repository))?;
    
    // Then I should see Home in favorites
    FinderAssert::new(&finder).has_favorites(&[Target::home()]);
    Ok(())
}

#[test]
fn shows_empty_favorites() -> Result<()> {
    // Given favorites are empty
    let repository = TestSidebarRepository::new();
    repository.save(&Sidebar::default())?;
    
    // When I start Finder
    let finder = Finder::start(Box::new(repository))?;
    
    // Then I should see no favorites
    FinderAssert::new(&finder).is_empty();
    Ok(())
}

#[test]
fn shows_multiple_items_in_favorites() -> Result<()> {
    // Given Home and AirDrop are in favorites
    let repository = TestSidebarRepository::new();
    let mut sidebar = Sidebar::default();
    sidebar.favorites_mut().add(SidebarItem::home());
    sidebar.favorites_mut().add(SidebarItem::airdrop());
    repository.save(&sidebar)?;
    
    // When I start Finder
    let finder = Finder::start(Box::new(repository))?;
    
    // Then I should see both items in favorites
    FinderAssert::new(&finder).has_favorites(&[
        Target::home(),
        Target::airdrop(),
    ]);
    Ok(())
}

#[test]
fn shows_existing_favorites_when_opened() -> Result<()> {
    // Given I have some favorites in Finder
    let repository = TestSidebarRepository::new();
    let mut previous_finder = Finder::new(Box::new(repository.clone()));
    previous_finder.sidebar_mut().favorites_mut().add(SidebarItem::home());
    previous_finder.quit()?;
    
    // When I start Finder again
    let finder = Finder::start(Box::new(repository))?;
    
    // Then I should see my favorites
    let items = finder.sidebar().favorites().items();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].label(), "Home");
    Ok(())
}
