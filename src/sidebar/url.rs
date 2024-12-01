use super::SidebarUrl;
use core_foundation::{
    base::TCFType,
    string::CFString,
    url::{CFURLGetString, CFURL},
};

const SYSTEM_URL_PREFIX: &str = "com-apple-sfl://";
const REMOTE_DISC_IDENTIFIER: &str = "IsRemoteDisc";

pub struct UrlHandler;

impl UrlHandler {
    pub fn parse_url(url: &CFURL) -> Option<SidebarUrl> {
        let url_string = Self::get_url_string(url);

        match url_string {
            s if s.starts_with(SYSTEM_URL_PREFIX) => {
                if s.contains(REMOTE_DISC_IDENTIFIER) {
                    Some(SidebarUrl::RemoteDisc)
                } else {
                    Some(SidebarUrl::SystemUrl(s))
                }
            }
            s if s.starts_with("nwnode://") => Some(SidebarUrl::AirDrop),
            _ => url.to_path().map(SidebarUrl::File),
        }
    }

    fn get_url_string(url: &CFURL) -> String {
        unsafe {
            let url_str = CFURLGetString(url.as_concrete_TypeRef());
            CFString::wrap_under_create_rule(url_str).to_string()
        }
    }
}
