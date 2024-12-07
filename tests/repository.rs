use favkit::Repository;
use test_utils::MockMacOsApi;

mod test_utils;

#[test]
fn test_load_returns_empty_sidebar_when_favorites_list_is_null() {
    let home_dir = dirs::home_dir().unwrap();
    let mock_api = MockMacOsApi::with_favorites(vec![], home_dir);
    let repository = Repository::new(Box::new(mock_api));
    let sidebar = repository.load();
    assert!(sidebar.favorites().is_empty());
}
