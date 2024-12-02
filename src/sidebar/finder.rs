use crate::sidebar::cf::{CFList, CoreServicesOperations};
use crate::sidebar::error::{Result, SidebarError};
use crate::sidebar::{SidebarItem, SidebarOperations, SidebarUrl};
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
        let url = CFList::url_from_path(Path::new(path))?;
        self.list.add_url(url)
    }

    fn remove_item(&self, path: &str) -> Result<()> {
        let target_path = Path::new(path);
        let items = self.list.get_items()?;
        for item in items {
            if let Ok(SidebarUrl::File(item_path)) = item.parse_url() {
                if item_path == target_path {
                    return self.list.remove_item(&item);
                }
            }
        }
        Err(SidebarError::RemoveItem(format!(
            "Item not found: {}",
            path
        )))
    }
}
