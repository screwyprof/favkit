use std::path::{Path, PathBuf};
use crate::finder::sidebar::Target;

/// A MacOS-specific implementation for converting paths to Targets
pub struct MacOsPath(PathBuf);

impl From<PathBuf> for MacOsPath {
    fn from(path: PathBuf) -> Self {
        Self(path)
    }
}

impl TryFrom<MacOsPath> for Target {
    type Error = String;

    fn try_from(path: MacOsPath) -> Result<Self, Self::Error> {
        match path.0.as_path() {
            p if dirs::document_dir().is_some_and(|d| p == d.as_path()) => 
                Ok(Target::Documents(p.to_path_buf())),
            p if dirs::download_dir().is_some_and(|d| p == d.as_path()) => 
                Ok(Target::Downloads(p.to_path_buf())),
            p if dirs::home_dir().is_some_and(|d| p == d.as_path()) => 
                Ok(Target::Home(p.to_path_buf())),
            p if p == Path::new("/Applications") => 
                Ok(Target::Applications(p.to_path_buf())),
            p => Ok(Target::UserPath(p.to_path_buf()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_special_directories() {
        // Test Applications directory
        let apps = MacOsPath::from(PathBuf::from("/Applications"));
        assert!(matches!(
            Target::try_from(apps).unwrap(),
            Target::Applications(_)
        ));

        // Test user path
        let custom = MacOsPath::from(PathBuf::from("/custom/path"));
        assert!(matches!(
            Target::try_from(custom).unwrap(),
            Target::UserPath(_)
        ));
    }
}
