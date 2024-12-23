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
    Downloads,
    Desktop,
    Custom(String),
}

impl MacOsUrl {
    const AIRDROP: &'static str = "nwnode://domain-AirDrop";
    const RECENTS: &'static str = "file:///System/Library/CoreServices/Finder.app/Contents/Resources/MyLibraries/myDocuments.cannedSearch/";
    const APPLICATIONS: &'static str = "file:///Applications/";
    const USER_PREFIX: &'static str = "file:///Users/";
    const DOWNLOADS: &'static str = "Downloads";
    const DESKTOP: &'static str = "Desktop";

    fn clean_path(url: impl AsRef<str>) -> String {
        url.as_ref()
            .strip_prefix("file://")
            .and_then(|p| p.strip_suffix('/'))
            .unwrap_or(url.as_ref())
            .to_string()
    }

    fn is_user_folder(url: impl AsRef<str>, folder: impl AsRef<str>) -> bool {
        url.as_ref()
            .strip_prefix(Self::USER_PREFIX)
            .and_then(|rest| {
                let mut parts = rest.split('/').take(2);
                let _username = parts.next()?;
                let folder_name = parts.next()?;
                (folder_name == folder.as_ref()).then_some(())
            })
            .is_some()
    }
}

impl From<Url> for MacOsUrl {
    fn from(url: Url) -> Self {
        let url_str = url.to_string();

        match url_str.as_str() {
            Self::AIRDROP => Self::AirDrop,
            Self::RECENTS => Self::Recents,
            Self::APPLICATIONS => Self::Applications,
            url if Self::is_user_folder(url, Self::DESKTOP) => Self::Desktop,
            url if Self::is_user_folder(url, Self::DOWNLOADS) => Self::Downloads,
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
            MacOsUrl::Downloads => Target::Downloads,
            MacOsUrl::Desktop => Target::Desktop,
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
    fn should_convert_downloads_url() {
        let target = Target::from(FavoriteItem::new(
            create_url("file:///Users/user/Downloads/"),
            create_display_name("Downloads"),
        ));
        assert_eq!(target, Target::Downloads);
    }

    #[test]
    fn should_convert_desktop_url() {
        let target = Target::from(FavoriteItem::new(
            create_url("file:///Users/user/Desktop/"),
            create_display_name("Desktop"),
        ));
        assert_eq!(target, Target::Desktop);
    }

    #[test]
    fn should_convert_custom_url() {
        let target = Target::from(FavoriteItem::new(
            create_url("file:///Users/user/Documents/"),
            create_display_name("Documents"),
        ));
        assert_eq!(target, Target::Custom {
            label: "Documents".to_string(),
            path: "/Users/user/Documents".to_string(),
        });
    }

    #[test]
    fn should_not_recognize_deep_downloads_path_as_downloads() {
        let target = Target::from(FavoriteItem::new(
            create_url("file:///Users/user/Projects/Downloads/"),
            create_display_name("Downloads"),
        ));
        assert!(matches!(target, Target::Custom { .. }));
    }

    #[test]
    fn should_format_favorite_item() {
        let item = FavoriteItem::new(
            create_url("file:///Users/user/Documents/"),
            create_display_name("Documents"),
        );
        assert_eq!(
            format!("{}", item),
            "Documents -> file:///Users/user/Documents/"
        );
    }
}
