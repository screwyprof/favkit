mod error;
pub mod sidebar;

pub use error::{Error, Result};
pub use sidebar::{
    MacOsApi, MacOsLocation, MacOsPath, RealMacOsApi, Sidebar, SidebarApi, SidebarItem,
};
