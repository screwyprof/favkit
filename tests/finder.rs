use favkit::{
    finder::{Finder, FinderError, Result, SidebarItem, Target},
    system::favorites::FavoritesError,
};
use pretty_assertions::assert_eq;

use crate::mock::{favorites::FavoritesBuilder, mac_os_api::MockMacOsApi};

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
    let expected_result: Vec<SidebarItem> = vec![];
    let api = MockMacOsApi::new();
    let call_tracker = api.call_tracker();
    let finder = Finder::new(api);

    // Act
    let result = finder.get_favorites_list()?;

    // Assert
    assert_eq!(result, expected_result);
    call_tracker.verify();
    Ok(())
}

#[test]
fn should_handle_airdrop_item() -> Result<()> {
    // Arrange
    let expected_result = vec![SidebarItem::new(Target::AirDrop)];
    let favorites = FavoritesBuilder::new()
        .add_item(None, constants::AIRDROP_URL)
        .build();
    let api = MockMacOsApi::with_favorites(favorites);
    let call_tracker = api.call_tracker();
    let finder = Finder::new(api);

    // Act
    let result = finder.get_favorites_list()?;

    // Assert
    assert_eq!(result, expected_result);
    call_tracker.verify();
    Ok(())
}

#[test]
fn should_handle_recents_item() -> Result<()> {
    // Arrange
    let expected_result = vec![SidebarItem::new(Target::Recents)];
    let favorites = FavoritesBuilder::new()
        .add_item(Some(constants::RECENTS_LABEL), constants::RECENTS_URL)
        .build();
    let api = MockMacOsApi::with_favorites(favorites);
    let call_tracker = api.call_tracker();
    let finder = Finder::new(api);

    // Act
    let result = finder.get_favorites_list()?;

    // Assert
    assert_eq!(result, expected_result);
    call_tracker.verify();
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
    let api = MockMacOsApi::with_favorites(favorites);
    let call_tracker = api.call_tracker();
    let finder = Finder::new(api);

    // Act
    let result = finder.get_favorites_list()?;

    // Assert
    assert_eq!(result, expected_result);
    call_tracker.verify();
    Ok(())
}

#[test]
fn should_handle_multiple_favorites() -> Result<()> {
    // Arrange
    let expected_result = vec![
        SidebarItem::new(Target::AirDrop),
        SidebarItem::new(Target::Applications),
        SidebarItem::new(Target::Custom {
            label: constants::PROJECTS_LABEL.to_string(),
            path: constants::PROJECTS_PATH.to_string(),
        }),
    ];
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
    let result = finder.get_favorites_list()?;

    // Assert
    assert_eq!(result, expected_result);
    call_tracker.verify();
    Ok(())
}

#[test]
fn should_handle_custom_location() -> Result<()> {
    // Arrange
    let expected_result = vec![SidebarItem::new(Target::Custom {
        label: constants::PROJECTS_LABEL.to_string(),
        path: constants::PROJECTS_PATH.to_string(),
    })];

    let favorites = FavoritesBuilder::new()
        .add_item(Some(constants::PROJECTS_LABEL), constants::PROJECTS_URL)
        .build();
    let api = MockMacOsApi::with_favorites(favorites);
    let call_tracker = api.call_tracker();
    let finder = Finder::new(api);

    // Act
    let result = finder.get_favorites_list()?;

    // Assert
    assert_eq!(result, expected_result);
    call_tracker.verify();
    Ok(())
}
