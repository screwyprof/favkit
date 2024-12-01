use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub struct SidebarItem {
    pub name: String,
    pub url: SidebarUrl,
}

#[derive(Debug)]
pub enum SidebarUrl {
    File(PathBuf),
    AirDrop,
    SystemUrl(String),
    NotFound,
}

impl fmt::Display for SidebarUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SidebarUrl::File(path) => write!(f, "file://{}", path.display()),
            SidebarUrl::AirDrop => write!(f, "nwnode://domain-AirDrop"),
            SidebarUrl::SystemUrl(url) => write!(f, "{}", url),
            SidebarUrl::NotFound => write!(f, "NOTFOUND"),
        }
    }
}
