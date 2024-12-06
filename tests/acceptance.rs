use core_foundation::{array::CFArray, url::CFURL, base::TCFType};
use core_services::{LSSharedFileListRef, LSSharedFileListItemRef};
use favkit::finder::{
    Finder,
    SidebarRepository,
    Target,
};
use favkit::macos::test_utils::MockMacOsApi;

mod test_doubles {
    use super::*;

    pub struct FinderAssert {
        finder: Finder,
    }

    impl FinderAssert {
        pub fn new(finder: Finder) -> Self {
            Self { finder }
        }

        pub fn has_home_in_favorites(&self) -> bool {
            let favorites = self.finder.sidebar().favorites();
            favorites.iter().any(|item| item.path() == Target::home().path())
        }

        pub fn has_airdrop_in_favorites(&self) -> bool {
            let favorites = self.finder.sidebar().favorites();
            favorites.iter().any(|item| item.path() == Target::airdrop().path())
        }
    }
}

use test_doubles::FinderAssert;

#[test]
fn loads_home_and_airdrop() {
    // Given
    let home_url = CFURL::from_path(Target::home().path(), true).unwrap();
    let airdrop_url = CFURL::from_path(Target::airdrop().path(), true).unwrap();

    let item1 = 1 as LSSharedFileListItemRef;
    let item2 = 2 as LSSharedFileListItemRef;
    let items = vec![item1, item2];
    let items_array = CFArray::from_copyable(&items);

    let api = MockMacOsApi::new()
        .with_favorites_list(1 as LSSharedFileListRef)
        .with_favorites_snapshot(items_array)
        .with_item_url(home_url.as_concrete_TypeRef(), item1)
        .with_item_url(airdrop_url.as_concrete_TypeRef(), item2);

    let repo = SidebarRepository::new(api);
    let finder = Finder::new(repo);

    // When
    let finder = FinderAssert::new(finder);

    // Then
    assert!(finder.has_home_in_favorites());
    assert!(finder.has_airdrop_in_favorites());
}
