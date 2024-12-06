use crate::errors::Result;
use super::Sidebar;

pub trait SidebarRepository {
    fn load(&self) -> Result<Sidebar>;
    fn save(&self, sidebar: &Sidebar) -> Result<()>;
}
