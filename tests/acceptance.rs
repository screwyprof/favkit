#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use coverage_helper::test;
use favkit::FinderApi;

#[test]
fn should_get_favorites_list() {
    // Given I have Finder with MacOS API
    let finder = FinderApi::default();

    // When I list favorites
    let result = finder.get_favorites_list();

    // Then I get a list of favorites
    assert!(result.is_ok());
}
