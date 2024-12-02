use crate::sidebar::cf::{CFItem, CoreServicesOperations};
use crate::sidebar::error::{Result, SidebarError};
use crate::sidebar::url::UrlHandler;
use crate::sidebar::{SidebarItem, SidebarOperations, SidebarUrl};
use core_foundation::{
    array::CFArray,
    base::{CFType, TCFType},
    string::CFStringRef,
    url::CFURL,
};
use core_services::{LSSharedFileListItemRef, LSSharedFileListRef};
use std::path::Path;

pub struct FinderSidebar<'a> {
    list: LSSharedFileListRef,
    core_services: &'a dyn CoreServicesOperations,
}

impl<'a> FinderSidebar<'a> {
    pub(crate) fn new(
        list_type: CFStringRef,
        core_services: &'a dyn CoreServicesOperations,
    ) -> Result<Self> {
        let list = unsafe {
            core_services.create_list(list_type).ok_or_else(|| {
                SidebarError::CreateList("Failed to create shared file list".into())
            })?
        };

        if list.is_null() {
            return Err(SidebarError::CreateList(
                "Shared file list pointer is null".into(),
            ));
        }

        Ok(Self {
            list,
            core_services,
        })
    }

    fn get_items(&self) -> Result<CFArray<CFType>> {
        unsafe {
            self.core_services
                .copy_snapshot(self.list)
                .ok_or_else(|| SidebarError::Snapshot("Failed to get items snapshot".into()))
        }
    }

    fn parse_item(&self, item: &CFType) -> Option<SidebarItem> {
        println!("DEBUG: Starting parse_item with: {:?}", item);
        let item_ref = item.as_CFTypeRef() as LSSharedFileListItemRef;
        println!("DEBUG: Got item_ref: {:?}", item_ref);

        let cf_item = CFItem::new(item_ref, self.core_services);
        println!("DEBUG: Created CFItem: {:?}", cf_item);

        // Get URL and parse it
        println!("DEBUG: Getting resolved URL");
        let url = cf_item
            .resolved_url()
            .and_then(|url| {
                println!("DEBUG: Got URL: {:?}", url);
                let handler = UrlHandler::new(url);
                println!("DEBUG: Created URL handler");
                handler.parse().ok()
            })
            .unwrap_or(SidebarUrl::NotFound);
        println!("DEBUG: Parsed URL: {:?}", url);

        // Get name with special handling for known items
        let name = match &url {
            SidebarUrl::AirDrop => String::from("AirDrop"),
            SidebarUrl::RemoteDisc => String::from("Remote Disc"),
            _ => {
                println!("DEBUG: Getting display name");
                cf_item.display_name()?
            }
        };
        println!("DEBUG: Got name: {}", name);

        if name.is_empty() {
            println!("DEBUG: Empty name, returning None");
            return None;
        }

        println!("DEBUG: Creating SidebarItem");
        Some(SidebarItem { name, url })
    }
}

impl SidebarOperations for FinderSidebar<'_> {
    fn list_items(&self) -> Result<Vec<SidebarItem>> {
        let items = self.get_items()?;
        Ok(items
            .iter()
            .filter_map(|item| self.parse_item(&item))
            .collect())
    }

    fn add_item(&self, path: &str) -> Result<()> {
        let url = CFURL::from_path(Path::new(path), true)
            .ok_or_else(|| SidebarError::AddItem("Failed to create URL from path".into()))?;

        unsafe {
            self.core_services.insert_item(self.list, &url);
        }
        Ok(())
    }

    fn remove_item(&self, path: &str) -> Result<()> {
        let target_path = Path::new(path);
        if !target_path.exists() {
            return Err(SidebarError::RemoveItem(format!(
                "Path does not exist: {}",
                path
            )));
        }

        let items = self.get_items()?;

        for item in items.iter() {
            let item_ref = item.as_CFTypeRef() as LSSharedFileListItemRef;
            if let Some(url) = unsafe { self.core_services.copy_resolved_url(item_ref) } {
                if let Some(item_path) = url.to_path() {
                    if item_path == target_path {
                        unsafe {
                            self.core_services.remove_item(self.list, item_ref);
                        }
                        return Ok(());
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
