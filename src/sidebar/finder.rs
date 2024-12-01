use crate::sidebar::cf::{CFItem, SharedFileList};
use crate::sidebar::error::{Result, SidebarError};
use crate::sidebar::url::UrlHandler;
use crate::sidebar::{SidebarItem, SidebarOperations, SidebarUrl};
use core_foundation::{
    array::CFArray,
    base::{CFType, TCFType},
    string::CFStringRef,
    url::CFURL,
};
use core_services::{LSSharedFileListInsertItemURL, LSSharedFileListItemRemove};
use std::path::Path;

pub struct FinderSidebar {
    list: SharedFileList,
}

impl FinderSidebar {
    pub fn new(list_type: CFStringRef) -> Result<Self> {
        let list = unsafe { SharedFileList::new(list_type) }
            .ok_or_else(|| SidebarError::CreateList("Failed to create shared file list".into()))?;
        Ok(Self { list })
    }

    fn get_items(&self) -> Result<CFArray<CFType>> {
        unsafe {
            let mut seed: u32 = 0;
            let items_ptr =
                core_services::LSSharedFileListCopySnapshot(self.list.as_raw(), &mut seed);
            if items_ptr.is_null() {
                return Err(SidebarError::Snapshot(
                    "Failed to get items snapshot".into(),
                ));
            }
            Ok(CFArray::wrap_under_create_rule(items_ptr.cast()))
        }
    }

    fn parse_item(&self, item: &CFType) -> Option<SidebarItem> {
        let item_ref = item.as_concrete_TypeRef() as *mut std::ffi::c_void;
        let cf_item = CFItem::new(item_ref.cast());

        // Get URL and parse it
        let url = cf_item
            .resolved_url()
            .and_then(|url| {
                let handler = UrlHandler::new(url);
                handler.parse().ok()
            })
            .unwrap_or(SidebarUrl::NotFound);

        // Get name with special handling for known items
        let name = match &url {
            SidebarUrl::AirDrop => String::from("AirDrop"),
            SidebarUrl::RemoteDisc => String::from("Remote Disc"),
            _ => cf_item.display_name()?,
        };

        if name.is_empty() {
            return None;
        }

        Some(SidebarItem { name, url })
    }
}

impl SidebarOperations for FinderSidebar {
    fn list_items(&self) -> Result<Vec<SidebarItem>> {
        let items = self.get_items()?;
        Ok(items
            .iter()
            .filter_map(|item| self.parse_item(&item))
            .collect())
    }

    fn add_item(&self, path: &str) -> Result<()> {
        let path = Path::new(path);
        if !path.exists() {
            return Err(SidebarError::invalid_path(path));
        }

        let url = CFURL::from_path(path, true)
            .ok_or_else(|| SidebarError::AddItem("Failed to create URL from path".into()))?;

        unsafe {
            LSSharedFileListInsertItemURL(
                self.list.as_raw(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                url.as_concrete_TypeRef(),
                std::ptr::null(),
                std::ptr::null_mut(),
            );
        }

        Ok(())
    }

    fn remove_item(&self, path: &str) -> Result<()> {
        let target_path = Path::new(path);
        if !target_path.exists() {
            return Err(SidebarError::invalid_path(target_path));
        }

        let items = self.get_items()?;
        for item in items.iter() {
            unsafe {
                let item_ref = item.as_concrete_TypeRef() as *mut std::ffi::c_void;
                let cf_item = CFItem::new(item_ref.cast());

                if let Some(url) = cf_item.resolved_url() {
                    if let Some(item_path) = url.to_path() {
                        if item_path == target_path {
                            LSSharedFileListItemRemove(self.list.as_raw(), item_ref.cast());
                            return Ok(());
                        }
                    }
                }
            }
        }

        Err(SidebarError::item_not_found(target_path))
    }
}
