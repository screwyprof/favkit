use crate::sidebar::cf::{CFList, CoreServicesOperations};
use crate::sidebar::error::{Result, SidebarError};
use crate::sidebar::{SidebarItem, SidebarOperations, SidebarUrl};
use core_foundation::url::CFURL;
use std::path::Path;

#[derive(Debug)]
pub struct FinderSidebar<'a> {
    list: CFList<'a>,
}

impl<'a> FinderSidebar<'a> {
    pub fn new_favorites(core_services: &'a dyn CoreServicesOperations) -> Result<Self> {
        Ok(Self {
            list: CFList::new_favorites(core_services)?,
        })
    }

    pub fn new_volumes(core_services: &'a dyn CoreServicesOperations) -> Result<Self> {
        Ok(Self {
            list: CFList::new_volumes(core_services)?,
        })
    }
}

impl SidebarOperations for FinderSidebar<'_> {
    fn list_items(&self) -> Result<Vec<SidebarItem>> {
        let items = self.list.get_items()?;
        Ok(items
            .iter()
            .filter_map(|item| {
                let url = item.parse_url().ok()?;
                let name = match &url {
                    SidebarUrl::AirDrop => String::from("AirDrop"),
                    SidebarUrl::RemoteDisc => String::from("Remote Disc"),
                    _ => item.display_name()?,
                };

                if name.is_empty() {
                    None
                } else {
                    Some(SidebarItem { name, url })
                }
            })
            .collect())
    }

    fn add_item(&self, path: &str) -> Result<()> {
        let url = CFURL::from_path(Path::new(path), true)
            .ok_or_else(|| SidebarError::AddItem("Failed to create URL from path".into()))?;
        self.list.add_url(url)
    }

    fn remove_item(&self, path: &str) -> Result<()> {
        let target_path = Path::new(path);
        let items = self.list.get_items()?;
        for item in items {
            if let Some(url) = item.resolved_url() {
                if let Some(item_path) = url.to_path() {
                    if item_path == target_path {
                        return self.list.remove_item(&item);
                    }
                }
            }
        }
        Err(SidebarError::RemoveItem(format!(
            "Item not found: {}",
            path
        )))
    }
}
