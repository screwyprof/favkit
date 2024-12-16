#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use coverage_helper::test;
use favkit::FinderApi;

#[test]
fn should_get_favorites_list() {
    let api = FinderApi;
    let result = api.get_favorites_list();
    assert!(result.is_ok());
}
