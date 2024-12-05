use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    AirDrop,
    Home(PathBuf),
}

impl Target {
    pub fn label(&self) -> &str {
        match self {
            Target::AirDrop => "AirDrop",
            Target::Home(_) => "Home",
        }
    }

    pub fn path(&self) -> Option<PathBuf> {
        match self {
            Target::AirDrop => None,
            Target::Home(path) => Some(path.clone()),
        }
    }

    pub fn home() -> Self {
        Self::Home(PathBuf::from("~/"))
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
        assert_eq!(home.path(), Some(PathBuf::from("~/")));
    }
}
