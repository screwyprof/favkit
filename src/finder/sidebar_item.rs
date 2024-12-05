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

impl TryFrom<&str> for SidebarItem {
    type Error = FinderError;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            target: Target::try_from(path)?,
        })
    }
}

impl TryFrom<String> for SidebarItem {
    type Error = FinderError;

    fn try_from(path: String) -> Result<Self, Self::Error> {
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

impl From<Target> for SidebarItem {
    fn from(target: Target) -> Self {
        Self { target }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use super::super::target::{AIRDROP_PATH, HOME_PATH};

    #[test]
    fn creates_airdrop_item() {
        let item = SidebarItem::airdrop();
        assert_eq!(item.label(), "AirDrop");
        assert_eq!(item.path(), Some(Path::new(AIRDROP_PATH)));
    }

    #[test]
    fn creates_home_item() {
        let item = SidebarItem::home();
        assert_eq!(item.label(), "Home");
        assert_eq!(item.path(), Some(Path::new(HOME_PATH)));
    }

    #[test]
    fn converts_home_from_str() {
        let item = SidebarItem::try_from(HOME_PATH).unwrap();
        assert_eq!(item.label(), "Home");
        assert_eq!(item.path(), Some(Path::new(HOME_PATH)));
    }

    #[test]
    fn converts_airdrop_from_str() {
        let item = SidebarItem::try_from(AIRDROP_PATH).unwrap();
        assert_eq!(item.label(), "AirDrop");
        assert_eq!(item.path(), Some(Path::new(AIRDROP_PATH)));
    }

    #[test]
    fn fails_on_home_subdirectory() {
        let result = SidebarItem::try_from("~/Downloads");
        assert!(matches!(result, Err(FinderError::UnsupportedTarget(_))));
    }

    #[test]
    fn fails_on_unsupported_path() {
        let result = SidebarItem::try_from("/some/random/path");
        assert!(matches!(result, Err(FinderError::UnsupportedTarget(_))));
    }

    #[test]
    fn converts_home_from_string() {
        let item = SidebarItem::try_from(HOME_PATH.to_string()).unwrap();
        assert_eq!(item.label(), "Home");
        assert_eq!(item.path(), Some(Path::new(HOME_PATH)));
    }

    #[test]
    fn converts_airdrop_from_pathbuf() {
        let path = PathBuf::from(AIRDROP_PATH);
        let item = SidebarItem::try_from(path).unwrap();
        assert_eq!(item.label(), "AirDrop");
        assert_eq!(item.path(), Some(Path::new(AIRDROP_PATH)));
    }

    #[test]
    fn converts_home_from_path() {
        let path = Path::new(HOME_PATH);
        let item = SidebarItem::try_from(path).unwrap();
        assert_eq!(item.label(), "Home");
        assert_eq!(item.path(), Some(Path::new(HOME_PATH)));
    }

    #[test]
    fn fails_on_invalid_string() {
        let result = SidebarItem::try_from("invalid/path".to_string());
        assert!(matches!(result, Err(FinderError::UnsupportedTarget(_))));
    }

    #[test]
    fn fails_on_invalid_pathbuf() {
        let result = SidebarItem::try_from(PathBuf::from("invalid/path"));
        assert!(matches!(result, Err(FinderError::UnsupportedTarget(_))));
    }

    #[test]
    fn try_from_str_works() {
        let item = SidebarItem::try_from(AIRDROP_PATH).unwrap();
        assert!(matches!(item.target, Target::AirDrop(_)));
    }

    #[test]
    fn try_from_str_fails_on_invalid_path() {
        let result = SidebarItem::try_from("/invalid/path");
        assert!(matches!(result, Err(FinderError::UnsupportedTarget(_))));
    }

    #[test]
    fn try_from_string_works() {
        let item = SidebarItem::try_from(AIRDROP_PATH.to_string()).unwrap();
        assert!(matches!(item.target, Target::AirDrop(_)));
    }

    #[test]
    fn try_from_string_fails_on_invalid_path() {
        let result = SidebarItem::try_from("/invalid/path".to_string());
        assert!(matches!(result, Err(FinderError::UnsupportedTarget(_))));
    }

    #[test]
    fn try_from_pathbuf_propagates_error() {
        let invalid_path = PathBuf::from("/invalid/path");
        let result = SidebarItem::try_from(invalid_path);
        assert!(matches!(result, Err(FinderError::UnsupportedTarget(_))));
    }

    #[test]
    fn try_from_path_propagates_error() {
        let invalid_path = Path::new("/invalid/path");
        let result = SidebarItem::try_from(invalid_path);
        assert!(matches!(result, Err(FinderError::UnsupportedTarget(_))));
    }

    #[test]
    fn try_from_string_propagates_error() {
        let invalid_path = String::from("/invalid/path");
        let result = SidebarItem::try_from(invalid_path);
        assert!(matches!(result, Err(FinderError::UnsupportedTarget(_))));
    }
}
