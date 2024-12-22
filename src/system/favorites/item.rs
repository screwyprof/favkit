use std::fmt;

use crate::{
    finder::Target,
    system::favorites::{DisplayName, Url},
};

pub struct FavoriteItem {
    url: Url,
    name: DisplayName,
}

impl FavoriteItem {
    const AIRDROP: &'static str = "nwnode://domain-AirDrop";
    const RECENTS: &'static str = "file:///System/Library/CoreServices/Finder.app/Contents/Resources/MyLibraries/myDocuments.cannedSearch/";
    const APPLICATIONS: &'static str = "file:///Applications/";

    pub fn new(url: Url, name: DisplayName) -> Self {
        Self { url, name }
    }

    fn is_special_folder(&self, folder: &str) -> bool {
        let path = self.url.to_string();
        let url_path = path.strip_prefix("file://").unwrap_or(&path);
        url_path.matches('/').count() == 4
            && url_path.ends_with(&format!("/{}/", folder))
            && url_path.starts_with("/Users/")
    }
}

impl fmt::Display for FavoriteItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}

impl From<FavoriteItem> for Target {
    fn from(path: FavoriteItem) -> Self {
        let url = path.to_string();
        match url.as_str() {
            FavoriteItem::AIRDROP => Target::AirDrop,
            FavoriteItem::RECENTS => Target::Recents,
            FavoriteItem::APPLICATIONS => Target::Applications,
            _ if path.is_special_folder("Downloads") => Target::Downloads,
            _ if path.is_special_folder("Desktop") => Target::Desktop,
            path_str => Target::Custom {
                label: path.name.to_string(),
                path: path_str.to_string(),
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
            create_url(FavoriteItem::AIRDROP),
            create_display_name("AirDrop"),
        ));
        assert_eq!(target, Target::AirDrop);
    }

    #[test]
    fn should_convert_recents_url() {
        let target = Target::from(FavoriteItem::new(
            create_url(FavoriteItem::RECENTS),
            create_display_name("Recents"),
        ));
        assert_eq!(target, Target::Recents);
    }

    #[test]
    fn should_convert_applications_url() {
        let target = Target::from(FavoriteItem::new(
            create_url(FavoriteItem::APPLICATIONS),
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
}
