use std::path::{Path, PathBuf};
use super::target::Target;
use crate::errors::FinderError;

#[derive(Debug, Clone)]
pub struct SidebarItem {
    path: PathBuf,
}

impl SidebarItem {
    pub fn new<P: AsRef<Path>>(path: P) -> Option<Self> {
        Self::validate_path(path).ok().map(|path| Self { path })
    }

    pub fn home() -> Self {
        Self::new(Target::home().path()).unwrap()
    }

    pub fn airdrop() -> Self {
        Self::new(Target::airdrop().path()).unwrap()
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn label(&self) -> String {
        if self.path == Target::home().path() {
            return "Home".to_string();
        }
        if self.path == Target::airdrop().path() {
            return "AirDrop".to_string();
        }
        self.path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown")
            .to_string()
    }

    fn validate_path<P: AsRef<Path>>(path: P) -> Result<PathBuf, FinderError> {
        let path = path.as_ref().to_path_buf();
        println!("Validating path: {:?}", path);
        println!("Target home path: {:?}", Target::home().path());
        println!("Target airdrop path: {:?}", Target::airdrop().path());
        if path == Target::home().path() || path == Target::airdrop().path() {
            println!("Path is valid");
            Ok(path)
        } else {
            println!("Path is invalid");
            Err(FinderError::UnsupportedTarget(path))
        }
    }
}

impl From<Target> for SidebarItem {
    fn from(target: Target) -> Self {
        Self::new(target.path()).unwrap()
    }
}

impl TryFrom<&str> for SidebarItem {
    type Error = FinderError;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        let path = Self::validate_path(path)?;
        Ok(Self::new(path).unwrap())
    }
}

impl TryFrom<PathBuf> for SidebarItem {
    type Error = FinderError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let path = Self::validate_path(path)?;
        Ok(Self::new(path).unwrap())
    }
}

impl TryFrom<&Path> for SidebarItem {
    type Error = FinderError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let path = Self::validate_path(path)?;
        Ok(Self::new(path).unwrap())
    }
}

impl TryFrom<String> for SidebarItem {
    type Error = FinderError;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        Self::try_from(path.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sidebar_item_from_home_path() {
        let home = Target::home();
        let item = SidebarItem::new(home.path()).unwrap();
        assert_eq!(item.path(), &home.path());
        assert_eq!(item.label(), "Home");
    }

    #[test]
    fn test_create_sidebar_item_from_airdrop_path() {
        let airdrop = Target::airdrop();
        let item = SidebarItem::new(airdrop.path()).unwrap();
        assert_eq!(item.path(), &airdrop.path());
        assert_eq!(item.label(), "AirDrop");
    }

    #[test]
    fn test_create_sidebar_item_from_invalid_path() {
        let path = PathBuf::from("/invalid/path");
        let result = SidebarItem::new(path.clone());
        assert!(result.is_none());
    }

    #[test]
    fn test_create_sidebar_item_from_str() {
        let home = Target::home();
        let item = SidebarItem::try_from(home.path().to_str().unwrap()).unwrap();
        assert_eq!(item.path(), &home.path());
    }

    #[test]
    fn test_create_sidebar_item_from_string() {
        let home = Target::home();
        let item = SidebarItem::try_from(home.path().to_str().unwrap().to_string()).unwrap();
        assert_eq!(item.path(), &home.path());
    }

    #[test]
    fn test_create_sidebar_item_from_target() {
        let home = Target::home();
        let item: SidebarItem = home.clone().into();
        assert_eq!(item.path(), &home.path());
    }
}
