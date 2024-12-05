use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    AirDrop,
    Home(PathBuf),
}

impl Target {
    pub fn home() -> Self {
        "~/".into()
    }

    pub fn label(&self) -> &str {
        match self {
            Target::AirDrop => "AirDrop",
            Target::Home(_) => "Home",
        }
    }

    pub fn path(&self) -> Option<&Path> {
        match self {
            Target::AirDrop => None,
            Target::Home(path) => Some(path.as_path()),
        }
    }
}

impl<T: AsRef<Path>> From<T> for Target {
    fn from(path: T) -> Self {
        Self::Home(path.as_ref().to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn airdrop_has_correct_label() {
        assert_eq!(Target::AirDrop.label(), "AirDrop");
        assert_eq!(Target::AirDrop.path(), None);
    }

    #[test]
    fn home_has_correct_label_and_path() {
        let home = Target::home();
        assert_eq!(home.label(), "Home");
        assert!(home.path().unwrap().ends_with("~/"));
    }

    #[test]
    fn converts_from_str() {
        let target = Target::from("~/Documents");
        assert_eq!(target.label(), "Home");
        assert!(target.path().unwrap().ends_with("Documents"));
    }

    #[test]
    fn converts_from_pathbuf() {
        let path = PathBuf::from("~/Downloads");
        let target = Target::from(path);
        assert_eq!(target.label(), "Home");
        assert!(target.path().unwrap().ends_with("Downloads"));
    }
}
