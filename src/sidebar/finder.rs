use super::{SidebarItem, SidebarOperations, SidebarUrl};
use crate::sidebar::cf::CFWrapper;
use crate::sidebar::error::{Result, SidebarError};
use crate::sidebar::url::UrlHandler;
use core_foundation::{
    array::CFArray,
    base::{CFType, TCFType},
    string::CFStringRef,
    url::CFURL,
};
use core_services::{
    LSSharedFileListCreate, LSSharedFileListInsertItemURL, LSSharedFileListItemRef,
    LSSharedFileListItemRemove, LSSharedFileListRef,
};
use std::path::PathBuf;

pub struct FinderSidebar {
    list: LSSharedFileListRef,
}

impl FinderSidebar {
    pub unsafe fn new(list_type: CFStringRef) -> Result<Self> {
        let list = LSSharedFileListCreate(std::ptr::null(), list_type, std::ptr::null());
        if list.is_null() {
            return Err(SidebarError::CreateList);
        }
        Ok(Self { list })
    }

    unsafe fn get_items(&self) -> Result<CFArray<CFType>> {
        let mut seed: u32 = 0;
        let items_ptr = core_services::LSSharedFileListCopySnapshot(self.list, &mut seed);
        if items_ptr.is_null() {
            return Err(SidebarError::Snapshot);
        }
        Ok(CFArray::wrap_under_create_rule(items_ptr.cast()))
    }
}

impl SidebarOperations for FinderSidebar {
    fn list_items(&self) -> Result<Vec<SidebarItem>> {
        unsafe {
            let items = self.get_items()?;
            let mut result = Vec::new();

            for item in items.iter() {
                let item_ref =
                    item.as_concrete_TypeRef() as *mut std::ffi::c_void as LSSharedFileListItemRef;

                let url = CFWrapper::get_url(item_ref)
                    .and_then(|url| UrlHandler::parse_url(&url))
                    .unwrap_or(SidebarUrl::NotFound);

                // Get name, with special handling for AirDrop
                let name = match url {
                    SidebarUrl::AirDrop => String::from("AirDrop"),
                    _ => CFWrapper::get_name(item_ref).unwrap_or_default(),
                };

                // Skip items with no name (except AirDrop which we just named)
                if name.is_empty() {
                    continue;
                }

                result.push(SidebarItem { name, url });
            }

            Ok(result)
        }
    }

    fn add_item(&self, path: &str) -> Result<()> {
        let url = CFURL::from_path(path, true)
            .ok_or_else(|| SidebarError::InvalidPath(path.to_string()))?;

        unsafe {
            LSSharedFileListInsertItemURL(
                self.list,
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
        let target_path = PathBuf::from(path);

        for item in self.list_items()? {
            if let SidebarUrl::File(item_path) = item.url {
                if item_path == target_path {
                    unsafe {
                        let items = self.get_items()?;
                        for item in items.iter() {
                            let item_ref = item.as_concrete_TypeRef() as LSSharedFileListItemRef;
                            if let Some(url) = CFWrapper::get_url(item_ref) {
                                if let Some(p) = url.to_path() {
                                    if p == target_path {
                                        LSSharedFileListItemRemove(self.list, item_ref);
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Err(SidebarError::ItemNotFound(path.to_string()))
    }
}
