use crate::{
    finder::Target,
    system::favorites::{DisplayName, Url},
};

pub struct TargetUrl(pub Url, pub DisplayName);

const AIRDROP_URL: &str = "nwnode://domain-AirDrop";
const RECENTS_URL: &str = "file:///System/Library/CoreServices/Finder.app/Contents/Resources/MyLibraries/myDocuments.cannedSearch/";
const APPLICATIONS_URL: &str = "file:///Applications/";

fn is_downloads_url(url: &str) -> bool {
    let url_path = url.strip_prefix("file://").unwrap_or(url);
    url_path.matches('/').count() == 4
        && url_path.ends_with("/Downloads/")
        && url_path.starts_with("/Users/")
}

fn is_desktop_url(url: &str) -> bool {
    let url_path = url.strip_prefix("file://").unwrap_or(url);
    url_path.matches('/').count() == 4
        && url_path.ends_with("/Desktop/")
        && url_path.starts_with("/Users/")
}

impl From<TargetUrl> for Target {
    fn from(target: TargetUrl) -> Self {
        let url = target.0.to_string();

        match url.as_str() {
            AIRDROP_URL => Target::AirDrop,
            RECENTS_URL => Target::Recents,
            APPLICATIONS_URL => Target::Applications,
            path if is_downloads_url(path) => Target::Downloads,
            path if is_desktop_url(path) => Target::Desktop,
            path => Target::Custom {
                label: target.1.to_string(),
                path: path.to_string(),
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
        let target = Target::from(TargetUrl(
            create_url(AIRDROP_URL),
            create_display_name("AirDrop"),
        ));
        assert_eq!(target, Target::AirDrop);
    }

    #[test]
    fn should_convert_recents_url() {
        let target = Target::from(TargetUrl(
            create_url(RECENTS_URL),
            create_display_name("Recents"),
        ));
        assert_eq!(target, Target::Recents);
    }

    #[test]
    fn should_convert_applications_url() {
        let target = Target::from(TargetUrl(
            create_url(APPLICATIONS_URL),
            create_display_name("Applications"),
        ));
        assert_eq!(target, Target::Applications);
    }

    #[test]
    fn should_convert_downloads_url() {
        let target = Target::from(TargetUrl(
            create_url("file:///Users/user/Downloads/"),
            create_display_name("Downloads"),
        ));
        assert_eq!(target, Target::Downloads);
    }

    #[test]
    fn should_convert_custom_url() {
        let target = Target::from(TargetUrl(
            create_url("file:///Users/user/Documents/"),
            create_display_name("Documents"),
        ));
        assert_eq!(target, Target::Custom {
            label: "Documents".to_string(),
            path: "file:///Users/user/Documents/".to_string(),
        });
    }

    #[test]
    fn should_convert_desktop_url() {
        let target = Target::from(TargetUrl(
            create_url("file:///Users/user/Desktop/"),
            create_display_name("Desktop"),
        ));
        assert_eq!(target, Target::Desktop);
    }
}
