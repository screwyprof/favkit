use crate::sidebar::error::{Result, SidebarError};
use crate::sidebar::SidebarUrl;
use core_foundation::{
    base::TCFType,
    string::CFString,
    url::{CFURLGetString, CFURL},
};
use std::path::PathBuf;

#[derive(Debug)]
pub struct UrlHandler {
    url: CFURL,
}

impl UrlHandler {
    pub fn new(url: CFURL) -> Self {
        Self { url }
    }

    pub fn parse(&self) -> Result<SidebarUrl> {
        let url_string = unsafe {
            let str_ref = CFURLGetString(self.url.as_concrete_TypeRef());
            CFString::wrap_under_get_rule(str_ref).to_string()
        };

        match url_string {
            s if s.starts_with("nwnode://") && s.contains("domain-AirDrop") => {
                Ok(SidebarUrl::AirDrop)
            }
            s if s.starts_with("com-apple-sfl://") && s.contains("IsRemoteDisc") => {
                Ok(SidebarUrl::RemoteDisc)
            }
            s if s.starts_with("file://") => {
                if let Some(path) = self.url.to_path() {
                    Ok(SidebarUrl::File(path))
                } else {
                    Ok(SidebarUrl::NotFound)
                }
            }
            s => Ok(SidebarUrl::SystemUrl(s)),
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
