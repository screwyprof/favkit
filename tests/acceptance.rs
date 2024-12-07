use anyhow::Result;
use favkit::{
    Repository,
    Target,
};
use std::path::PathBuf;

mod test_utils;
use test_utils::MockMacOsApi;

#[test]
fn it_shows_favorites() -> Result<()> {
    // Given
    let home_dir = dirs::home_dir().unwrap();
    let expected = vec![
        Target::AirDrop("nwnode://domain-AirDrop".to_string()),
        Target::Recents(PathBuf::from("/System/Library/CoreServices/Finder.app/Contents/Resources/MyLibraries/myDocuments.cannedSearch")),
        Target::Applications(PathBuf::from("/Applications")),
        Target::Desktop(home_dir.join("Desktop")),
        Target::Downloads(home_dir.join("Downloads")),
        Target::Home(home_dir.clone()),
        Target::CustomPath(PathBuf::from("/Users/happygopher/Projects")),
    ];
    
    let api = MockMacOsApi::with_favorites(expected.clone(), home_dir);
    let repository = Repository::new(Box::new(api));
    let sidebar = repository.load();

    // When
    let favorites: Vec<Target> = sidebar
        .favorites()
        .iter()
        .map(|item| item.target().clone())
        .collect();

    // Then
    assert_eq!(favorites, expected);

    Ok(())
}
