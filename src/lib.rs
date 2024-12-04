mod error;
pub mod sidebar;

pub use error::{Error, Result};
pub use sidebar::{MacOsApi, MacOsLocation, RealMacOsApi, Sidebar, SidebarApi, SidebarItem};
