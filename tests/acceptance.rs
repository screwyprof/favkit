#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use coverage_helper::test;
use favkit::FinderApi;

#[test]
fn it_lists_finder_favorites() {
    // Given I have Finder with favorites
    let finder = FinderApi;

    // When I list favorites
    let favorites = finder.get_favorites_list();

    // Then I get a list of favorites
    assert!(!favorites.is_empty());
}
