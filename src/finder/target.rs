use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum TargetLocation {
    Path(PathBuf),
    Url(String),
}

impl TargetLocation {
    pub fn as_path(&self) -> Option<&Path> {
        match self {
            Self::Path(p) => Some(p),
            Self::Url(_) => None,
        }
    }

    pub fn as_url(&self) -> Option<&str> {
        match self {
            Self::Path(_) => None,
            Self::Url(u) => Some(u),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    Home(TargetLocation),
    Desktop(TargetLocation),
    Documents(TargetLocation),
    Downloads(TargetLocation),
    Applications(TargetLocation),
    AirDrop(TargetLocation),
    CustomPath(TargetLocation),
}

impl Target {
    pub fn location(&self) -> &TargetLocation {
        match self {
            Self::Home(loc) |
            Self::Desktop(loc) |
            Self::Documents(loc) |
            Self::Downloads(loc) |
            Self::Applications(loc) |
            Self::AirDrop(loc) |
            Self::CustomPath(loc) => loc,
        }
    }

    pub fn default_display_name(&self) -> &str {
        match self {
            Self::Home(_) => "Home",
            Self::Desktop(_) => "Desktop",
            Self::Documents(_) => "Documents",
            Self::Downloads(_) => "Downloads",
            Self::Applications(_) => "Applications",
            Self::AirDrop(_) => "AirDrop",
            Self::CustomPath(loc) => loc.as_path()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown"),
        }
    }
}