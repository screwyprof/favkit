use std::path::{Path, PathBuf};
use std::fmt;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub enum TargetLocation {
    Path(PathBuf),
    Url(String),
}

impl TryFrom<PathBuf> for TargetLocation {
    type Error = &'static str;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        if path.as_os_str().is_empty() {
            return Err("Path cannot be empty");
        }
        Ok(Self::Path(path))
    }
}

impl TryFrom<String> for TargetLocation {
    type Error = &'static str;

    fn try_from(url: String) -> Result<Self, Self::Error> {
        if !url.contains("://") {
            return Err("Invalid URL format");
        }
        Ok(Self::Url(url))
    }
}

impl TryFrom<&TargetLocation> for PathBuf {
    type Error = &'static str;

    fn try_from(location: &TargetLocation) -> Result<Self, Self::Error> {
        match location {
            TargetLocation::Path(p) => Ok(p.clone()),
            TargetLocation::Url(_) => Err("Cannot convert URL to Path"),
        }
    }
}

impl TryFrom<&TargetLocation> for String {
    type Error = &'static str;

    fn try_from(location: &TargetLocation) -> Result<Self, Self::Error> {
        match location {
            TargetLocation::Path(_) => Err("Cannot convert Path to URL"),
            TargetLocation::Url(u) => Ok(u.clone()),
        }
    }
}

impl AsRef<Path> for TargetLocation {
    fn as_ref(&self) -> &Path {
        match self {
            Self::Path(p) => p.as_ref(),
            Self::Url(_) => panic!("Cannot get path reference from URL"),
        }
    }
}

impl TargetLocation {
    pub fn is_path(&self) -> bool {
        matches!(self, Self::Path(_))
    }

    pub fn is_url(&self) -> bool {
        matches!(self, Self::Url(_))
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
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Home(_) => "Home",
            Self::Desktop(_) => "Desktop",
            Self::Documents(_) => "Documents",
            Self::Downloads(_) => "Downloads",
            Self::Applications(_) => "Applications",
            Self::AirDrop(_) => "AirDrop",
            Self::CustomPath(loc) => return match loc {
                TargetLocation::Path(p) => write!(f, "{}", p.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown")),
                TargetLocation::Url(u) => write!(f, "{}", u),
            },
        };
        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_location_conversions() {
        let path = PathBuf::from("/Users/test/Documents");
        let location = TargetLocation::try_from(path.clone()).unwrap();
        assert!(location.is_path());
        assert!(!location.is_url());

        let invalid_path = PathBuf::from("");
        assert!(TargetLocation::try_from(invalid_path).is_err());

        let url = String::from("nwnode://domain-AirDrop");
        let location = TargetLocation::try_from(url.clone()).unwrap();
        assert!(location.is_url());
        assert!(!location.is_path());

        let invalid_url = String::from("not-a-url");
        assert!(TargetLocation::try_from(invalid_url).is_err());

        let path_location = TargetLocation::try_from(path).unwrap();
        let converted_path: PathBuf = (&path_location).try_into().unwrap();
        assert_eq!(converted_path.file_name().unwrap(), "Documents");

        let url_location = TargetLocation::try_from(String::from("nwnode://domain-AirDrop")).unwrap();
        let converted_url: String = (&url_location).try_into().unwrap();
        assert_eq!(converted_url, "nwnode://domain-AirDrop");

        let path_location = TargetLocation::Path(PathBuf::from("/some/path"));
        let url_location = TargetLocation::Url(String::from("nwnode://domain-AirDrop"));

        assert!(String::try_from(&path_location).is_err());
        assert!(PathBuf::try_from(&url_location).is_err());
    }

    #[test]
    fn test_target_display() {
        // Test standard targets with their appropriate paths
        let home = TargetLocation::try_from(PathBuf::from("/Users/test")).unwrap();
        let desktop = TargetLocation::try_from(PathBuf::from("/Users/test/Desktop")).unwrap();
        let documents = TargetLocation::try_from(PathBuf::from("/Users/test/Documents")).unwrap();
        let downloads = TargetLocation::try_from(PathBuf::from("/Users/test/Downloads")).unwrap();
        let applications = TargetLocation::try_from(PathBuf::from("/Applications")).unwrap();
        let airdrop = TargetLocation::try_from(String::from("nwnode://domain-AirDrop")).unwrap();

        assert_eq!(Target::Home(home).to_string(), "Home");
        assert_eq!(Target::Desktop(desktop).to_string(), "Desktop");
        assert_eq!(Target::Documents(documents).to_string(), "Documents");
        assert_eq!(Target::Downloads(downloads).to_string(), "Downloads");
        assert_eq!(Target::Applications(applications).to_string(), "Applications");
        assert_eq!(Target::AirDrop(airdrop).to_string(), "AirDrop");

        // Test CustomPath with file path
        let custom_path = TargetLocation::try_from(PathBuf::from("/Users/test/custom.txt")).unwrap();
        let target = Target::CustomPath(custom_path);
        assert_eq!(target.to_string(), "custom.txt");

        // Test CustomPath with URL
        let url_location = TargetLocation::try_from(String::from("nwnode://custom-url")).unwrap();
        let target = Target::CustomPath(url_location);
        assert_eq!(target.to_string(), "nwnode://custom-url");
    }

    #[test]
    fn test_target_location_as_ref() {
        let path = PathBuf::from("/Users/test/Documents");
        let location = TargetLocation::try_from(path).unwrap();
        let path_ref: &Path = location.as_ref();
        assert_eq!(path_ref.file_name().unwrap(), "Documents");
    }

    #[test]
    #[should_panic(expected = "Cannot get path reference from URL")]
    fn test_target_location_as_ref_url_panics() {
        let location = TargetLocation::try_from(String::from("nwnode://domain-AirDrop")).unwrap();
        let _: &Path = location.as_ref();
    }
}