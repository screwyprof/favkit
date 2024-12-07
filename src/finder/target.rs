use std::path::PathBuf;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    AirDrop(String),
    Applications(PathBuf),
    Desktop(PathBuf),
    Documents(PathBuf),
    Downloads(PathBuf),
    Home(PathBuf),
    Recents(PathBuf),
    CustomPath(PathBuf),
}

impl Target {
    /// Returns a human-readable label for the target.
    pub fn label(&self) -> &str {
        match self {
            Self::Home(_) => "Home",
            Self::Desktop(_) => "Desktop",
            Self::Documents(_) => "Documents",
            Self::Downloads(_) => "Downloads",
            Self::Applications(_) => "Applications",
            Self::AirDrop(_) => "AirDrop",
            Self::Recents(_) => "Recents",
            Self::CustomPath(_) => "Custom Path",
        }
    }

    pub fn from_path(path_str: &str) -> Self {
        if path_str.starts_with("nwnode://domain-AirDrop") {
            return Self::AirDrop(path_str.to_string());
        }

        let path = PathBuf::from(path_str);
        
        // Check against home directory paths
        if let Some(home_dir) = dirs::home_dir() {
            if path == home_dir {
                return Self::Home(path);
            }
            if path == home_dir.join("Desktop") {
                return Self::Desktop(path);
            }
            if path == home_dir.join("Documents") {
                return Self::Documents(path);
            }
            if path == home_dir.join("Downloads") {
                return Self::Downloads(path);
            }
        }

        // Check for special paths
        if path == PathBuf::from("/Applications") {
            return Self::Applications(path);
        }
        if path == PathBuf::from("/System/Library/CoreServices/Finder.app/Contents/Resources/MyLibraries/myDocuments.cannedSearch") {
            return Self::Recents(path);
        }

        Self::CustomPath(path)
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Home(path) => write!(f, "{} ({})", self.label(), path.display()),
            Self::Desktop(path) => write!(f, "{} ({})", self.label(), path.display()),
            Self::Documents(path) => write!(f, "{} ({})", self.label(), path.display()),
            Self::Downloads(path) => write!(f, "{} ({})", self.label(), path.display()),
            Self::Applications(path) => write!(f, "{} ({})", self.label(), path.display()),
            Self::AirDrop(url) => write!(f, "{} ({})", self.label(), url),
            Self::Recents(path) => write!(f, "{} ({})", self.label(), path.display()),
            Self::CustomPath(path) => write!(f, "{} ({})", self.label(), path.display()),
        }
    }
}