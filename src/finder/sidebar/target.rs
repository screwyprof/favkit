use std::path::PathBuf;

/// Target represents a location that can be opened in Finder
#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    /// System-wide AirDrop location
    AirDrop(String), // URL like "nwnode://domain-AirDrop"
    /// User's Documents folder
    Documents(PathBuf),
    /// System Applications folder
    Applications(PathBuf),
    /// User's Downloads folder
    Downloads(PathBuf),
    /// User's Home folder
    Home(PathBuf),
    /// Custom user-specified path
    UserPath(PathBuf),
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl From<&Target> for String {
    fn from(target: &Target) -> Self {
        match target {
            Target::AirDrop(_) => "nwnode://domain-AirDrop".to_string(),
            Target::Documents(path) |
            Target::Applications(path) |
            Target::Downloads(path) |
            Target::Home(path) |
            Target::UserPath(path) => format!("file://{}", path.display()),
        }
    }
}
