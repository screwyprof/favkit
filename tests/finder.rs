use favkit::{
    finder::{Finder, FinderError, Result, SidebarItem, Target},
    system::favorites::FavoritesError,
};
use pretty_assertions::assert_eq;

mod mock;
use mock::{favorites::FavoritesBuilder, mac_os_api::MockMacOsApiBuilder};

mod constants {
    // Display Labels
    pub const RECENTS_LABEL: &str = "Recents";
    pub const APPLICATIONS_LABEL: &str = "Applications";
    pub const DOWNLOADS_LABEL: &str = "Downloads";
    pub const DOCUMENTS_LABEL: &str = "Documents";

    // URLs
    pub const AIRDROP_URL: &str = "nwnode://domain-AirDrop";
    pub const RECENTS_URL: &str = "file:///System/Library/CoreServices/Finder.app/Contents/Resources/MyLibraries/myDocuments.cannedSearch/";
    pub const APPLICATIONS_URL: &str = "file:///Applications/";
    pub const DOWNLOADS_URL: &str = "file:///Users/user/Downloads/";
    pub const DOCUMENTS_URL: &str = "file:///Users/user/Documents/";
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
        label: constants::DOCUMENTS_LABEL.to_string(),
        path: constants::DOCUMENTS_URL.to_string(),
    })];
    let favorites = FavoritesBuilder::new()
        .add_item(Some(constants::DOCUMENTS_LABEL), constants::DOCUMENTS_URL)
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
fn should_handle_recents_item() -> Result<()> {
    // Arrange
    let expected_result = vec![SidebarItem::new(Target::Recents)];
    let favorites = FavoritesBuilder::new()
        .add_item(Some(constants::RECENTS_LABEL), constants::RECENTS_URL)
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
fn should_handle_applications_item() -> Result<()> {
    // Arrange
    let expected_result = vec![SidebarItem::new(Target::Applications)];
    let favorites = FavoritesBuilder::new()
        .add_item(
            Some(constants::APPLICATIONS_LABEL),
            constants::APPLICATIONS_URL,
        )
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
fn should_handle_downloads_item() -> Result<()> {
    // Arrange
    let expected_result = vec![SidebarItem::new(Target::Downloads)];
    let favorites = FavoritesBuilder::new()
        .add_item(Some(constants::DOWNLOADS_LABEL), constants::DOWNLOADS_URL)
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
        SidebarItem::new(Target::Applications),
        SidebarItem::new(Target::Downloads),
    ];
    let favorites = FavoritesBuilder::new()
        .add_item(None, constants::AIRDROP_URL)
        .add_item(
            Some(constants::APPLICATIONS_LABEL),
            constants::APPLICATIONS_URL,
        )
        .add_item(Some(constants::DOWNLOADS_LABEL), constants::DOWNLOADS_URL)
        .build();
    let mock_api = MockMacOsApiBuilder::new().with_favorites(favorites).build();
    let finder = Finder::new(mock_api);

    // Act
    let result = finder.get_favorites_list()?;

    // Assert
    assert_eq!(result, expected_result);
    Ok(())
}
