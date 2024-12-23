use favkit::{
    finder::{Finder, FinderError, Result},
    system::favorites::FavoritesError,
};
use pretty_assertions::assert_eq;

use crate::mock::{
    favorites::{FavoritesBuilder, assert_favorites},
    mac_os_api::MockMacOsApi,
    sidebar_items::SidebarItemsBuilder,
};

mod mock;
mod constants {
    // AirDrop
    pub const AIRDROP_URL: &str = "nwnode://domain-AirDrop";

    // Recents
    pub const RECENTS_LABEL: &str = "Recents";
    pub const RECENTS_URL: &str = "file:///System/Library/CoreServices/Finder.app/Contents/Resources/MyLibraries/myDocuments.cannedSearch/";

    // Applications
    pub const APPLICATIONS_LABEL: &str = "Applications";
    pub const APPLICATIONS_URL: &str = "file:///Applications/";

    // Projects
    pub const PROJECTS_LABEL: &str = "Projects";
    pub const PROJECTS_PATH: &str = "/Users/user/Projects";
    pub const PROJECTS_URL: &str = "file:///Users/user/Projects/";
}

#[test]
fn should_fail_when_list_handle_is_null() -> Result<()> {
    // Arrange
    let expected_error = Err(FinderError::AccessError(FavoritesError::NullListHandle));

    let api = MockMacOsApi::with_null_list();
    let call_tracker = api.call_tracker();
    let finder = Finder::new(api);

    // Act
    let result = finder.get_favorites_list();

    // Assert
    assert_eq!(result, expected_error);
    call_tracker.verify();
    Ok(())
}

#[test]
fn should_fail_when_snapshot_handle_is_null() -> Result<()> {
    // Arrange
    let expected_error = Err(FinderError::AccessError(FavoritesError::NullSnapshotHandle));

    let api = MockMacOsApi::with_null_snapshot();
    let call_tracker = api.call_tracker();
    let finder = Finder::new(api);

    // Act
    let result = finder.get_favorites_list();

    // Assert
    assert_eq!(result, expected_error);
    call_tracker.verify();
    Ok(())
}

#[test]
fn should_return_empty_list_when_no_favorites() -> Result<()> {
    // Arrange
    let expected_result = SidebarItemsBuilder::new().build();

    let api = MockMacOsApi::new();
    let call_tracker = api.call_tracker();
    let finder = Finder::new(api);

    // Act
    let result = finder.get_favorites_list();

    // Assert
    assert_favorites(result, expected_result, &call_tracker);
    Ok(())
}

#[test]
fn should_handle_airdrop_item() -> Result<()> {
    // Arrange
    let expected_result = SidebarItemsBuilder::new().airdrop().build();

    let favorites = FavoritesBuilder::new()
        .add_item(None, constants::AIRDROP_URL)
        .build();

    let api = MockMacOsApi::with_favorites(favorites);
    let call_tracker = api.call_tracker();
    let finder = Finder::new(api);

    // Act
    let result = finder.get_favorites_list();

    // Assert
    assert_favorites(result, expected_result, &call_tracker);
    Ok(())
}

#[test]
fn should_handle_recents_item() -> Result<()> {
    // Arrange
    let expected_result = SidebarItemsBuilder::new().recents().build();

    let favorites = FavoritesBuilder::new()
        .add_item(Some(constants::RECENTS_LABEL), constants::RECENTS_URL)
        .build();

    let api = MockMacOsApi::with_favorites(favorites);
    let call_tracker = api.call_tracker();
    let finder = Finder::new(api);

    // Act
    let result = finder.get_favorites_list();

    // Assert
    assert_favorites(result, expected_result, &call_tracker);
    Ok(())
}

#[test]
fn should_handle_applications_item() -> Result<()> {
    // Arrange
    let expected_result = SidebarItemsBuilder::new().applications().build();

    let favorites = FavoritesBuilder::new()
        .add_item(
            Some(constants::APPLICATIONS_LABEL),
            constants::APPLICATIONS_URL,
        )
        .build();

    let api = MockMacOsApi::with_favorites(favorites);
    let call_tracker = api.call_tracker();
    let finder = Finder::new(api);

    // Act
    let result = finder.get_favorites_list();

    // Assert
    assert_favorites(result, expected_result, &call_tracker);
    Ok(())
}

#[test]
fn should_handle_custom_location() -> Result<()> {
    // Arrange
    let expected_result = SidebarItemsBuilder::new()
        .custom(constants::PROJECTS_LABEL, constants::PROJECTS_PATH)
        .build();

    let favorites = FavoritesBuilder::new()
        .add_item(Some(constants::PROJECTS_LABEL), constants::PROJECTS_URL)
        .build();

    let api = MockMacOsApi::with_favorites(favorites);
    let call_tracker = api.call_tracker();
    let finder = Finder::new(api);

    // Act
    let result = finder.get_favorites_list();

    // Assert
    assert_favorites(result, expected_result, &call_tracker);
    Ok(())
}

#[test]
fn should_handle_multiple_favorites() -> Result<()> {
    // Arrange
    let expected_result = SidebarItemsBuilder::new()
        .airdrop()
        .applications()
        .custom(constants::PROJECTS_LABEL, constants::PROJECTS_PATH)
        .build();

    let favorites = FavoritesBuilder::new()
        .add_item(None, constants::AIRDROP_URL)
        .add_item(
            Some(constants::APPLICATIONS_LABEL),
            constants::APPLICATIONS_URL,
        )
        .add_item(Some(constants::PROJECTS_LABEL), constants::PROJECTS_URL)
        .build();

    let api = MockMacOsApi::with_favorites(favorites);
    let call_tracker = api.call_tracker();
    let finder = Finder::new(api);

    // Act
    let result = finder.get_favorites_list();

    // Assert
    assert_favorites(result, expected_result, &call_tracker);
    Ok(())
}
