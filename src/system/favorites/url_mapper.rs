use crate::{
    finder::Target,
    system::favorites::{DisplayName, Url},
};

pub struct TargetUrl(pub Url, pub DisplayName);

enum MacOsUrl {
    AirDrop,
    Recents,
    Custom(String),
}

impl From<&str> for MacOsUrl {
    fn from(url: &str) -> Self {
        match url {
            "nwnode://domain-AirDrop" => MacOsUrl::AirDrop,
            "file:///System/Library/CoreServices/Finder.app/Contents/Resources/MyLibraries/myDocuments.cannedSearch/" => {
                MacOsUrl::Recents
            }
            path => MacOsUrl::Custom(path.to_string()),
        }
    }
}

impl From<TargetUrl> for Target {
    fn from(target: TargetUrl) -> Self {
        let url = target.0.to_string();
        match MacOsUrl::from(url.as_str()) {
            MacOsUrl::AirDrop => Target::AirDrop,
            MacOsUrl::Recents => Target::Recents,
            MacOsUrl::Custom(path) => Target::Custom {
                label: target.1.to_string(),
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
            create_url("nwnode://domain-AirDrop"),
            create_display_name("AirDrop"),
        ));
        assert_eq!(target, Target::AirDrop);
    }

    #[test]
    fn should_convert_recents_url() {
        let target = Target::from(TargetUrl(
            create_url(
                "file:///System/Library/CoreServices/Finder.app/Contents/Resources/MyLibraries/myDocuments.cannedSearch/",
            ),
            create_display_name("Recents"),
        ));
        assert_eq!(target, Target::Recents);
    }

    #[test]
    fn should_convert_custom_url() {
        let target = Target::from(TargetUrl(
            create_url("file:///Users/user/Documents"),
            create_display_name("Documents"),
        ));
        assert_eq!(target, Target::Custom {
            label: "Documents".to_string(),
            path: "file:///Users/user/Documents".to_string(),
        });
    }
}
