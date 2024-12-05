use favkit::error::Result;
use favkit::item::Item;

mod test_doubles {
    use super::*;

    pub struct TestFinder {
        items_to_return: Vec<Item>,
    }

    impl TestFinder {
        pub fn returning_items(items: Vec<Item>) -> Self {
            Self { 
                items_to_return: items 
            }
        }

        pub fn inspect(&self) -> Result<Vec<Item>> {
            Ok(self.items_to_return.clone())
        }
    }
}

use test_doubles::TestFinder;

#[test]
fn lists_empty_sidebar() -> Result<()> {
    // Given: Finder has no items in the sidebar
    let finder = TestFinder::returning_items(vec![]);
    
    // When: We request sidebar items
    let items = finder.inspect()?;
    
    // Then: We get an empty list
    assert!(items.is_empty(), "Should return empty list when sidebar has no items");
    
    Ok(())
}
