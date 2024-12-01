use crate::error::Result;
use crate::types::SidebarItem;

pub trait SidebarOperations {
    fn list_items(&self) -> Result<Vec<SidebarItem>>;
    fn add_item(&self, path: &str) -> Result<()>;
    fn remove_item(&self, path: &str) -> Result<()>;
}
