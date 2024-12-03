use core_foundation::{array::CFArray, base::TCFType, string::CFString};
use std::convert::TryFrom;

use super::{
    macos_api::MacOsApi,
    path::{CFURLWrapper, MacOsPath},
    SidebarItem,
};

pub struct SidebarApi<T: MacOsApi> {
    raw: T,
}

impl<T: MacOsApi> SidebarApi<T> {
    pub fn new(raw: T) -> Self {
        Self { raw }
    }

    pub fn list_favorite_items(&self) -> Vec<SidebarItem> {
        unsafe {
            let favorites_list = self.raw.create_favorites_list();
            if favorites_list.is_null() {
                return vec![];
            }

            let mut seed = 0;
            let items_ref = self.raw.copy_snapshot(favorites_list, &mut seed);
            let items = CFArray::<*const std::ffi::c_void>::wrap_under_create_rule(items_ref);

            items
                .iter()
                .filter_map(|item_ref| {
                    let item_ref = *item_ref as core_services::LSSharedFileListItemRef;

                    // Get item name
                    let name_ref = self.raw.copy_display_name(item_ref);
                    if name_ref.is_null() {
                        return None;
                    }
                    let name = CFString::wrap_under_create_rule(name_ref);

                    // Get item URL
                    let url_ref = self.raw.copy_resolved_url(item_ref);
                    if url_ref.is_null() {
                        return None;
                    }
                    let url = core_foundation::url::CFURL::wrap_under_create_rule(url_ref);

                    // Convert URL to MacOsPath using the wrapper
                    let path = MacOsPath::try_from(CFURLWrapper::from(&url)).ok()?;

                    Some(SidebarItem::new(name.to_string(), path))
                })
                .collect()
        }
    }
}

impl<T: MacOsApi + Default> Default for SidebarApi<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
