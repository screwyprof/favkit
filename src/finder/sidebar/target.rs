use std::path::PathBuf;
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum TargetError {
    #[error("Invalid path")]
    InvalidPath,
    #[error("Unknown special folder")]
    UnknownSpecialFolder,
}

impl TryFrom<&str> for Target {
    type Error = TargetError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "AirDrop" => Ok(Target::AirDrop("nwnode://domain-AirDrop".to_string())),
            "Documents" => Ok(Target::Documents(PathBuf::from("/Users/current/Documents"))),
            "Applications" => Ok(Target::Applications(PathBuf::from("/Applications"))),
            "Downloads" => Ok(Target::Downloads(PathBuf::from("/Users/current/Downloads"))),
            "Home" => Ok(Target::Home(PathBuf::from("/Users/current"))),
            path => Ok(Target::UserPath(PathBuf::from(path))),
        }
    }
}

impl Target {
    pub fn path(&self) -> Option<&PathBuf> {
        match self {
            Target::AirDrop(_) => None,
            Target::Documents(path) => Some(path),
            Target::Applications(path) => Some(path),
            Target::Downloads(path) => Some(path),
            Target::Home(path) => Some(path),
            Target::UserPath(path) => Some(path),
        }
    }

    pub fn as_path_buf(&self) -> Option<PathBuf> {
        self.path().cloned()
    }
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::AirDrop(url) => write!(f, "{}", url),
            Target::Documents(path) => write!(f, "{}", path.display()),
            Target::Applications(path) => write!(f, "{}", path.display()),
            Target::Downloads(path) => write!(f, "{}", path.display()),
            Target::Home(path) => write!(f, "{}", path.display()),
            Target::UserPath(path) => write!(f, "{}", path.display()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_creation() {
        // Test AirDrop with URL
        let target = Target::try_from("AirDrop").unwrap();
        assert!(matches!(target, Target::AirDrop(_)));
        if let Target::AirDrop(url) = target {
            assert_eq!(url, "nwnode://domain-AirDrop");
        }

        // Test Documents with path
        let target = Target::try_from("Documents").unwrap();
        assert!(matches!(target, Target::Documents(_)));
        if let Target::Documents(path) = target {
            assert_eq!(path, PathBuf::from("/Users/current/Documents"));
        }

        // Test custom path
        let target = Target::try_from("/custom/path").unwrap();
        assert!(matches!(target, Target::UserPath(_)));
        if let Target::UserPath(path) = target {
            assert_eq!(path, PathBuf::from("/custom/path"));
        }
    }
}
