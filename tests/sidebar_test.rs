use favkit::Sidebar;

#[test]
fn it_returns_empty_favorites_by_default() {
    let sidebar = Sidebar::new();
    let favorites = sidebar.favorites().list_items();

    assert!(favorites.is_empty());
}
