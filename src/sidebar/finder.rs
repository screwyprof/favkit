use crate::sidebar::cf::{CFItem, CoreServicesOperations, DefaultCoreServices};
use crate::sidebar::error::{Result, SidebarError};
use crate::sidebar::url::UrlHandler;
use crate::sidebar::{SidebarItem, SidebarOperations, SidebarUrl};
use core_foundation::{
    array::CFArray,
    base::{CFType, TCFType},
    string::CFStringRef,
    url::CFURL,
};
use core_services::LSSharedFileListRef;
use std::path::Path;

pub struct FinderSidebar {
    list: LSSharedFileListRef,
    core_services: Box<dyn CoreServicesOperations>,
}

impl FinderSidebar {
    pub fn new(list_type: CFStringRef) -> Result<Self> {
        Self::with_core_services(list_type, Box::new(DefaultCoreServices))
    }

    pub(crate) fn with_core_services(
        list_type: CFStringRef,
        core_services: Box<dyn CoreServicesOperations>,
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
        let item_ref = item.as_concrete_TypeRef() as *mut std::ffi::c_void;
        let cf_item = CFItem::new(item_ref.cast(), self.core_services.as_ref());

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
            let item_ref = item.as_CFTypeRef() as *const _ as *mut _;
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
