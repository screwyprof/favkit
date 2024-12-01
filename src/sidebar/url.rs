use crate::sidebar::error::{Result, SidebarError};
use crate::sidebar::SidebarUrl;
use core_foundation::{
    base::TCFType,
    string::CFString,
    url::{CFURLGetString, CFURL},
};
use std::path::PathBuf;

const SYSTEM_URL_PREFIX: &str = "com-apple-sfl://";
const REMOTE_DISC_IDENTIFIER: &str = "IsRemoteDisc";
const AIRDROP_PREFIX: &str = "nwnode://";
const AIRDROP_DOMAIN: &str = "domain-AirDrop";

#[derive(Debug)]
pub struct UrlHandler {
    url: CFURL,
}

impl UrlHandler {
    pub fn new(url: CFURL) -> Self {
        Self { url }
    }

    pub fn parse(&self) -> Result<SidebarUrl> {
        let url_string = self.get_url_string();

        match url_string {
            s if s.starts_with(SYSTEM_URL_PREFIX) => {
                if s.contains(REMOTE_DISC_IDENTIFIER) {
                    Ok(SidebarUrl::RemoteDisc)
                } else {
                    Ok(SidebarUrl::SystemUrl(s))
                }
            }
            s if s.starts_with(AIRDROP_PREFIX) && s.contains(AIRDROP_DOMAIN) => {
                Ok(SidebarUrl::AirDrop)
            }
            _ => self.parse_file_url(),
        }
    }

    fn parse_file_url(&self) -> Result<SidebarUrl> {
        self.url
            .to_path()
            .map(SidebarUrl::File)
            .ok_or_else(|| SidebarError::UrlResolution("Failed to resolve file URL".into()))
    }

    fn get_url_string(&self) -> String {
        unsafe {
            let url_str = CFURLGetString(self.url.as_concrete_TypeRef());
            CFString::wrap_under_create_rule(url_str).to_string()
        }
    }
}

impl SidebarUrl {
    pub fn from_path(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        if !path.exists() {
            return Err(SidebarError::invalid_path(path));
        }
        Ok(Self::File(path))
    }

    pub fn as_path(&self) -> Option<&PathBuf> {
        match self {
            Self::File(path) => Some(path),
            _ => None,
        }
    }
}

