use crate::{
    finder::Target,
    system::favorites::{DisplayName, Url},
};

#[derive(Debug)]
enum MacOsUrl {
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
    const USER_HOME_FOLDER_DEPTH: usize = 4; // /Users/username/folder/

    fn is_user_downloads(url: &str) -> bool {
        Self::is_user_folder(url, "Downloads")
    }

    fn is_user_desktop(url: &str) -> bool {
        Self::is_user_folder(url, "Desktop")
    }

    fn is_user_folder(url: &str, folder: &str) -> bool {
        let url_path = url.strip_prefix("file://").unwrap_or(url);
        url_path.matches('/').count() == Self::USER_HOME_FOLDER_DEPTH
            && url_path.ends_with(&format!("/{}/", folder))
            && url_path.starts_with("/Users/")
    }
}

impl From<&str> for MacOsUrl {
    fn from(url: &str) -> Self {
        match url {
            Self::AIRDROP => Self::AirDrop,
            Self::RECENTS => Self::Recents,
            Self::APPLICATIONS => Self::Applications,
            path if Self::is_user_desktop(path) => Self::Desktop,
            path if Self::is_user_downloads(path) => Self::Downloads,
            path => Self::Custom(path.to_string()),
        }
    }
}

pub struct FavoriteItem {
    url: Url,
    name: DisplayName,
}

impl FavoriteItem {
    pub fn new(url: Url, name: DisplayName) -> Self {
        Self { url, name }
    }
}

impl From<FavoriteItem> for Target {
    fn from(item: FavoriteItem) -> Self {
        let url = item.url.to_string();
        match MacOsUrl::from(url.as_str()) {
            MacOsUrl::AirDrop => Target::AirDrop,
            MacOsUrl::Recents => Target::Recents,
            MacOsUrl::Applications => Target::Applications,
            MacOsUrl::Downloads => Target::Downloads,
            MacOsUrl::Desktop => Target::Desktop,
            MacOsUrl::Custom(path) => Target::Custom {
                label: item.name.to_string(),
                path,
            },
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
            path: "file:///Users/user/Documents/".to_string(),
        });
    }

    #[test]
    fn should_not_recognize_deep_downloads_path() {
        let target = Target::from(FavoriteItem::new(
            create_url("file:///Users/user/Documents/Downloads/"),
            create_display_name("Downloads"),
        ));
        assert!(matches!(target, Target::Custom { .. }));
    }
}
