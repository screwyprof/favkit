use std::path::{Path, PathBuf};
use std::convert::TryFrom;
use super::target::Target;
use crate::errors::FinderError;

#[derive(Debug, Clone, PartialEq)]
pub struct SidebarItem {
    target: Target,
}

impl SidebarItem {
    pub fn home() -> Self {
        Target::home().into()
    }

    pub fn airdrop() -> Self {
        Target::airdrop().into()
    }

    pub fn label(&self) -> &str {
        self.target.label()
    }

    pub fn path(&self) -> Option<&Path> {
        Some(self.target.path())
    }
}

impl From<Target> for SidebarItem {
    fn from(target: Target) -> Self {
        Self { target }
    }
}

impl TryFrom<&str> for SidebarItem {
    type Error = FinderError;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            target: Target::try_from(path)?,
        })
    }
}

impl TryFrom<PathBuf> for SidebarItem {
    type Error = FinderError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        Ok(Self {
            target: Target::try_from(path)?,
        })
    }
}

impl TryFrom<&Path> for SidebarItem {
    type Error = FinderError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        Ok(Self {
            target: Target::try_from(path)?,
        })
    }
}

impl TryFrom<String> for SidebarItem {
    type Error = FinderError;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        Self::try_from(path.as_str())
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use super::super::target::{AIRDROP_PATH, HOME_PATH};

    mod constructors {
        use super::*;

        #[test]
        fn creates_airdrop() {
            let item = SidebarItem::airdrop();
            assert!(matches!(item.target, Target::AirDrop(_)));
            assert_eq!(item.label(), "AirDrop");
            assert_eq!(item.path(), Some(Path::new(AIRDROP_PATH)));
        }

        #[test]
        fn creates_home() {
            let item = SidebarItem::home();
            assert!(matches!(item.target, Target::Home(_)));
            assert_eq!(item.label(), "Home");
            assert_eq!(item.path().map(|p| p.to_path_buf()), Some(PathBuf::from(HOME_PATH)));
        }
    }

    mod conversions {
        use super::*;

        #[test]
        fn converts_from_str() {
            // AirDrop
            let item = SidebarItem::try_from(AIRDROP_PATH).unwrap();
            assert!(matches!(item.target, Target::AirDrop(_)));
            assert_eq!(item.label(), "AirDrop");
            assert_eq!(item.path(), Some(Path::new(AIRDROP_PATH)));

            // Home
            let item = SidebarItem::try_from(HOME_PATH).unwrap();
            assert!(matches!(item.target, Target::Home(_)));
            assert_eq!(item.label(), "Home");
            assert_eq!(item.path().map(|p| p.to_path_buf()), Some(PathBuf::from(HOME_PATH)));
        }

        #[test]
        fn converts_from_string() {
            // AirDrop
            let item = SidebarItem::try_from(AIRDROP_PATH.to_string()).unwrap();
            assert!(matches!(item.target, Target::AirDrop(_)));
            assert_eq!(item.label(), "AirDrop");
            assert_eq!(item.path(), Some(Path::new(AIRDROP_PATH)));

            // Home
            let item = SidebarItem::try_from(HOME_PATH.to_string()).unwrap();
            assert!(matches!(item.target, Target::Home(_)));
            assert_eq!(item.label(), "Home");
            assert_eq!(item.path().map(|p| p.to_path_buf()), Some(PathBuf::from(HOME_PATH)));
        }

        #[test]
        fn converts_from_pathbuf() {
            // AirDrop
            let item = SidebarItem::try_from(PathBuf::from(AIRDROP_PATH)).unwrap();
            assert!(matches!(item.target, Target::AirDrop(_)));
            assert_eq!(item.label(), "AirDrop");
            assert_eq!(item.path(), Some(Path::new(AIRDROP_PATH)));

            // Home
            let item = SidebarItem::try_from(PathBuf::from(HOME_PATH)).unwrap();
            assert!(matches!(item.target, Target::Home(_)));
            assert_eq!(item.label(), "Home");
            assert_eq!(item.path().map(|p| p.to_path_buf()), Some(PathBuf::from(HOME_PATH)));
        }

        #[test]
        fn converts_from_path() {
            // AirDrop
            let item = SidebarItem::try_from(Path::new(AIRDROP_PATH)).unwrap();
            assert!(matches!(item.target, Target::AirDrop(_)));
            assert_eq!(item.label(), "AirDrop");
            assert_eq!(item.path(), Some(Path::new(AIRDROP_PATH)));

            // Home
            let item = SidebarItem::try_from(Path::new(HOME_PATH)).unwrap();
            assert!(matches!(item.target, Target::Home(_)));
            assert_eq!(item.label(), "Home");
            assert_eq!(item.path().map(|p| p.to_path_buf()), Some(PathBuf::from(HOME_PATH)));
        }
    }

    mod errors {
        use super::*;

        #[test]
        fn rejects_unsupported_path() {
            // String
            let result = SidebarItem::try_from("/invalid/path".to_string());
            assert!(matches!(result, Err(FinderError::UnsupportedTarget(_))));

            // &str
            let result = SidebarItem::try_from("/invalid/path");
            assert!(matches!(result, Err(FinderError::UnsupportedTarget(_))));

            // PathBuf
            let result = SidebarItem::try_from(PathBuf::from("/invalid/path"));
            assert!(matches!(result, Err(FinderError::UnsupportedTarget(_))));

            // &Path
            let result = SidebarItem::try_from(Path::new("/invalid/path"));
            assert!(matches!(result, Err(FinderError::UnsupportedTarget(_))));
        }

        #[test]
        fn rejects_home_subdirectory() {
            let result = SidebarItem::try_from("~/Downloads");
            assert!(matches!(result, Err(FinderError::UnsupportedTarget(_))));
        }
    }
}
