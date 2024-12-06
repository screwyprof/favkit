use favkit::finder::{Finder, Target, SidebarRepository};
use core_foundation::{
    array::CFArray,
    base::TCFType,
    url::CFURL,
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use favkit::finder::macos::test_utils::MockMacOsApi;

mod test_doubles {
    use super::*;

    pub struct FinderAssert {
        pub finder: Finder,
    }

    impl FinderAssert {
        pub fn new(finder: Finder) -> Self {
            Self { finder }
        }

        pub fn has_home_in_favorites(&self) -> bool {
            self.finder
                .sidebar()
                .favorites()
                .items()
                .iter()
                .any(|item| item.path() == Target::home().path())
        }

        pub fn has_airdrop_in_favorites(&self) -> bool {
            self.finder
                .sidebar()
                .favorites()
                .items()
                .iter()
                .any(|item| item.path() == Target::airdrop().path())
        }
    }
}

use test_doubles::FinderAssert;

#[test]
fn loads_empty_favorites() {
    // Given
    let api = MockMacOsApi::new()
        .with_favorites_list(1 as LSSharedFileListRef)
        .with_favorites_snapshot(CFArray::from_copyable(&[]));

    let repo = SidebarRepository::new(api);
    let finder = Finder::new(repo);

    // When
    let finder = FinderAssert::new(finder);

    // Then
    assert!(!finder.has_home_in_favorites());
    assert!(!finder.has_airdrop_in_favorites());
}

#[test]
fn loads_home_and_airdrop() {
    // Given
    let home_url = CFURL::from_path(Target::home().path(), true).unwrap();
    let airdrop_url = CFURL::from_path(Target::airdrop().path(), true).unwrap();

    let api = MockMacOsApi::new()
        .with_favorites_list(1 as LSSharedFileListRef)
        .with_favorites_snapshot(CFArray::from_copyable(&[
            1 as LSSharedFileListItemRef,
            2 as LSSharedFileListItemRef,
        ]))
        .with_item_url(home_url.as_concrete_TypeRef(), 1 as LSSharedFileListItemRef)
        .with_item_url(airdrop_url.as_concrete_TypeRef(), 2 as LSSharedFileListItemRef);

    let repo = SidebarRepository::new(api);
    let finder = Finder::new(repo);

    // When
    let finder = FinderAssert::new(finder);

    // Then
    assert!(finder.has_home_in_favorites());
    assert!(finder.has_airdrop_in_favorites());
}
