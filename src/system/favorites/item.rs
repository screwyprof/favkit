use std::fmt;

use crate::{
    finder::Target,
    system::favorites::{DisplayName, Url},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacOsUrl {
    AirDrop,
    Recents,
    Applications,
    Custom(String),
}

impl MacOsUrl {
    const AIRDROP: &'static str = "nwnode://domain-AirDrop";
    const RECENTS: &'static str = "file:///System/Library/CoreServices/Finder.app/Contents/Resources/MyLibraries/myDocuments.cannedSearch/";
    const APPLICATIONS: &'static str = "file:///Applications/";

    fn clean_path(url: impl AsRef<str>) -> String {
        url.as_ref()
            .strip_prefix("file://")
            .and_then(|p| p.strip_suffix('/'))
            .unwrap_or(url.as_ref())
            .to_string()
    }
}

impl From<Url> for MacOsUrl {
    fn from(url: Url) -> Self {
        let url_str = url.to_string();

        match url_str.as_str() {
            Self::AIRDROP => Self::AirDrop,
            Self::RECENTS => Self::Recents,
            Self::APPLICATIONS => Self::Applications,
            url => Self::Custom(Self::clean_path(url)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FavoriteItem {
    url: Url,
    name: DisplayName,
}

impl FavoriteItem {
    pub fn new(url: Url, name: DisplayName) -> Self {
        Self { url, name }
    }
}

impl fmt::Display for FavoriteItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.name, self.url)
    }
}

impl From<FavoriteItem> for Target {
    fn from(item: FavoriteItem) -> Self {
        match MacOsUrl::from(item.url) {
            MacOsUrl::AirDrop => Target::AirDrop,
            MacOsUrl::Recents => Target::Recents,
            MacOsUrl::Applications => Target::Applications,
            MacOsUrl::Custom(path) => Target::custom(item.name.to_string(), path),
        }
    }
}

#[cfg(test)]
mod tests {
    use core_foundation::{
        base::TCFType,
        string::CFString,
        url::{CFURL, kCFURLPOSIXPathStyle},
    };
    use pretty_assertions::assert_eq;

    use super::*;

    fn create_url(path: &str) -> Url {
        let cf_string = CFString::new(path);
        let is_dir = path.ends_with('/');
        let cf_url = CFURL::from_file_system_path(cf_string, kCFURLPOSIXPathStyle, is_dir);
        Url::try_from(cf_url.as_concrete_TypeRef()).unwrap()
    }

    fn create_display_name(name: &str) -> DisplayName {
        let cf_string = CFString::new(name);
        DisplayName::try_from(cf_string.as_concrete_TypeRef()).unwrap()
    }

    #[test]
    fn should_convert_airdrop_url() {
        let target = Target::from(FavoriteItem::new(
            create_url(MacOsUrl::AIRDROP),
            create_display_name("AirDrop"),
        ));
        assert_eq!(target, Target::AirDrop);
    }

    #[test]
    fn should_convert_recents_url() {
        let target = Target::from(FavoriteItem::new(
            create_url(MacOsUrl::RECENTS),
            create_display_name("Recents"),
        ));
        assert_eq!(target, Target::Recents);
    }

    #[test]
    fn should_convert_applications_url() {
        let target = Target::from(FavoriteItem::new(
            create_url(MacOsUrl::APPLICATIONS),
            create_display_name("Applications"),
        ));
        assert_eq!(target, Target::Applications);
    }

    #[test]
    fn should_convert_custom_url() {
        let target = Target::from(FavoriteItem::new(
            create_url("file:///Users/user/Projects/"),
            create_display_name("Projects"),
        ));
        assert_eq!(target, Target::Custom {
            label: "Projects".to_string(),
            path: "/Users/user/Projects".to_string(),
        });
    }

    #[test]
    fn should_format_favorite_item() {
        let item = FavoriteItem::new(
            create_url("file:///Users/user/Projects/"),
            create_display_name("Projects"),
        );
        assert_eq!(
            format!("{}", item),
            "Projects -> file:///Users/user/Projects/"
        );
    }
}
