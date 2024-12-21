use favkit::{
    finder::{Finder, FinderError, Result, SidebarItem, Target},
    system::favorites::FavoritesError,
};
use pretty_assertions::assert_eq;

mod mock;
use mock::{favorites::FavoritesBuilder, mac_os_api::MockMacOsApiBuilder};

mod constants {
    pub const DOCUMENTS_NAME: &str = "Documents";
    pub const DOCUMENTS_PATH: &str = "/Users/user/Documents/";
    pub const AIRDROP_URL: &str = "nwnode://domain-AirDrop";
}

#[test]
fn should_fail_when_list_handle_is_null() -> Result<()> {
    // Arrange
    let expected_error = Err(FinderError::AccessError(FavoritesError::NullListHandle));
    let mock_api = MockMacOsApiBuilder::new().with_null_list().build();
    let finder = Finder::new(mock_api);

    // Act
    let result = finder.get_favorites_list();

    // Assert
    assert_eq!(result, expected_error);
    Ok(())
}

#[test]
fn should_fail_when_snapshot_handle_is_null() -> Result<()> {
    // Arrange
    let expected_error = Err(FinderError::AccessError(FavoritesError::NullSnapshotHandle));
    let mock_api = MockMacOsApiBuilder::new().with_null_snapshot().build();
    let finder = Finder::new(mock_api);

    // Act
    let result = finder.get_favorites_list();

    // Assert
    assert_eq!(result, expected_error);
    Ok(())
}

#[test]
fn should_return_empty_list_when_no_favorites() -> Result<()> {
    // Arrange
    let expected_result: Vec<SidebarItem> = vec![];
    let mock_api = MockMacOsApiBuilder::new().build();
    let finder = Finder::new(mock_api);

    // Act
    let result = finder.get_favorites_list()?;

    // Assert
    assert_eq!(result, expected_result);
    Ok(())
}

#[test]
fn should_return_favorite_with_display_name_and_url() -> Result<()> {
    // Arrange
    let expected_result = vec![SidebarItem::new(Target::Custom {
        label: constants::DOCUMENTS_NAME.to_string(),
        path: format!("file://{}", constants::DOCUMENTS_PATH),
    })];
    let favorites = FavoritesBuilder::new()
        .add_item(Some(constants::DOCUMENTS_NAME), constants::DOCUMENTS_PATH)
        .build();
    let mock_api = MockMacOsApiBuilder::new().with_favorites(favorites).build();
    let finder = Finder::new(mock_api);

    // Act
    let result = finder.get_favorites_list()?;

    // Assert
    assert_eq!(result, expected_result);
    Ok(())
}

#[test]
fn should_handle_airdrop_item() -> Result<()> {
    // Arrange
    let expected_result = vec![SidebarItem::new(Target::AirDrop)];
    let favorites = FavoritesBuilder::new()
        .add_item(None, constants::AIRDROP_URL)
        .build();
    let mock_api = MockMacOsApiBuilder::new().with_favorites(favorites).build();
    let finder = Finder::new(mock_api);

    // Act
    let result = finder.get_favorites_list()?;

    // Assert
    assert_eq!(result, expected_result);
    Ok(())
}

#[test]
fn should_handle_multiple_favorites() -> Result<()> {
    // Arrange
    let expected_result = vec![
        SidebarItem::new(Target::AirDrop),
        SidebarItem::new(Target::Custom {
            label: "Applications".to_string(),
            path: "file:///Applications/".to_string(),
        }),
        SidebarItem::new(Target::Custom {
            label: "Downloads".to_string(),
            path: "file:///Users/user/Downloads/".to_string(),
        }),
    ];
    let favorites = FavoritesBuilder::new()
        .add_item(None, constants::AIRDROP_URL)
        .add_item(Some("Applications"), "/Applications/")
        .add_item(Some("Downloads"), "/Users/user/Downloads/")
        .build();
    let mock_api = MockMacOsApiBuilder::new().with_favorites(favorites).build();
    let finder = Finder::new(mock_api);

    // Act
    let result = finder.get_favorites_list()?;

    // Assert
    assert_eq!(result, expected_result);
    Ok(())
}
