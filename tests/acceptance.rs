use favkit::Finder;

#[test]
fn should_get_favorites_list() {
    // Given I have Finder with MacOS API
    let finder = Finder::default();

    // When I list favorites
    let result = finder.get_favorites_list();

    // Then I get a list of favorites
    assert!(result.is_ok());
}
