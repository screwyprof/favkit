mod errors;

use crate::system::api::MacOsApi;
pub use errors::Result;

use core_foundation::base::kCFAllocatorDefault;
use core_services::kLSSharedFileListFavoriteItems;

pub struct FinderApi<'a, T: MacOsApi> {
    macos_api: &'a T,
}

impl<'a, T: MacOsApi> FinderApi<'a, T> {
    pub fn new(macos_api: &'a T) -> Self {
        Self { macos_api }
    }

    pub fn get_favorites_list(&self) -> Result<Vec<String>> {
        unsafe {
            let _list = self.macos_api.ls_shared_file_list_create(
                kCFAllocatorDefault,
                kLSSharedFileListFavoriteItems,
                std::ptr::null(),
            );

            Ok(vec![])
        }
    }
}
