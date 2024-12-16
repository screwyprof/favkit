#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use coverage_helper::test;
use favkit::{Favorites, FinderApi, RealMacOsApi};

#[test]
fn should_get_favorites_list() {
    // Given I have Finder with MacOS API
    let macos_api = RealMacOsApi::new();
    let favorites = Favorites::new(&macos_api);
    let finder = FinderApi::new(&favorites);

    // When I list favorites
    let result = finder.get_favorites_list();

    // Then I get a list of favorites
    assert!(result.is_ok());
}
