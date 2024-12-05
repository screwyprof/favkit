use std::path::{Path, PathBuf};
use std::convert::TryFrom;
use crate::errors::FinderError;

pub const AIRDROP_PATH: &str = "nwnode://domain-AirDrop";
pub const HOME_PATH: &str = "~/";

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    AirDrop(PathBuf),
    Home(PathBuf),
}

impl Target {
    pub fn airdrop() -> Self {
        Self::AirDrop(PathBuf::from(AIRDROP_PATH))
    }

    pub fn home() -> Self {
        Self::Home(PathBuf::from(HOME_PATH))
    }

    pub fn label(&self) -> &str {
        match self {
            Self::AirDrop(_) => "AirDrop",
            Self::Home(_) => "Home",
        }
    }

    pub fn path(&self) -> &Path {
        match self {
            Self::AirDrop(path) => path,
            Self::Home(path) => path,
        }
    }

    fn try_from_path(path: impl AsRef<Path>) -> Result<Self, FinderError> {
        let path = path.as_ref();

        if path.to_str().map_or(false, |s| s == AIRDROP_PATH) {
            return Ok(Self::AirDrop(path.to_path_buf()));
        }

        let path_str = path.to_str()
            .ok_or_else(|| FinderError::invalid_path(path))?;

        if path_str == HOME_PATH {
            return Ok(Self::Home(path.to_path_buf()));
        }

        Err(FinderError::UnsupportedTarget(path.to_path_buf()))
    }
}

impl TryFrom<&str> for Target {
    type Error = FinderError;

    fn try_from(path: &str) -> Result<Self, Self::Error> {
        Self::try_from_path(path)
    }
}

impl TryFrom<PathBuf> for Target {
    type Error = FinderError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        Self::try_from_path(path)
    }
}

impl TryFrom<&Path> for Target {
    type Error = FinderError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        Self::try_from_path(path)
    }
}

impl TryFrom<String> for Target {
    type Error = FinderError;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        Self::try_from_path(path)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use coverage_helper::test;

    #[test]
    fn airdrop_has_correct_label() {
        let target = Target::airdrop();
        assert_eq!(target.label(), "AirDrop");
    }

    #[test]
    fn recognizes_home_path() {
        let target = Target::home();
        assert!(matches!(target, Target::Home(_)));
        assert_eq!(target.label(), "Home");
        assert_eq!(target.path(), Path::new(HOME_PATH));
    }

    #[test]
    fn recognizes_airdrop_path() {
        let target = Target::try_from_path(Path::new(AIRDROP_PATH)).unwrap();
        assert!(matches!(target, Target::AirDrop(_)));
        assert_eq!(target.label(), "AirDrop");
        assert_eq!(target.path(), Path::new(AIRDROP_PATH));
    }

    #[test]
    fn fails_on_non_utf8_path() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        let invalid_utf8 = OsStr::from_bytes(&[0x80]);
        let path = Path::new(invalid_utf8);

        let result = Target::try_from_path(path);
        assert!(matches!(
            result,
            Err(FinderError::InvalidPath { path: _, source: None })
        ));
    }

    #[test]
    fn fails_on_unsupported_path() {
        let result = Target::try_from("/some/random/path");
        assert!(matches!(result, Err(FinderError::UnsupportedTarget(_))));
    }

    #[test]
    fn fails_on_empty_path() {
        let result = Target::try_from_path(Path::new(""));
        assert!(matches!(result, Err(FinderError::UnsupportedTarget(_))));
    }
}
