use core_foundation::url::{CFURLRef, CFURL};
use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use crate::finder::target::Target;
use std::ptr;

/// Convert a CFURLRef to a Target
/// 
/// # Safety
/// 
/// The caller must ensure that:
/// - The `url` parameter is either null or a valid CFURLRef
/// - The CFURLRef must remain valid for the duration of this function call
pub unsafe fn url_to_target(url: CFURLRef) -> Target {
    if url.is_null() {
        return Target::Home(dirs::home_dir().unwrap_or_default());
    }

    let url = CFURL::wrap_under_get_rule(url);
    
    // Special handling for AirDrop
    let cf_string = url.get_string();
    let url_string = cf_string.to_string();
    if url_string.contains("nwnode://domain-AirDrop") {
        return Target::AirDrop(url_string);
    }

    // Handle regular file paths
    let path = url.to_path().unwrap_or_default();
    let path_str = path.to_string_lossy();
    
    // Remove the "file://" prefix if present
    let path_str = path_str.trim_start_matches("file://");
    Target::from_path(path_str)
}

/// Convert a Target to a CFURLRef
/// 
/// # Safety
/// 
/// This function is unsafe because it creates a Core Foundation URL that must be properly released.
pub unsafe fn target_to_url(target: &Target) -> CFURLRef {
    match target {
        Target::AirDrop(url) => {
            let cf_str = CFString::new(url);
            core_foundation::url::CFURLCreateWithString(
                core_foundation::base::kCFAllocatorDefault,
                cf_str.as_concrete_TypeRef(),
                ptr::null(),
            )
        }
        _ => {
            let path = match target {
                Target::Home(path) |
                Target::Desktop(path) |
                Target::Documents(path) |
                Target::CustomPath(path) => path,
                _ => unreachable!(),
            };
            let url = format!("file://{}", path.display());
            let cf_str = CFString::new(&url);
            core_foundation::url::CFURLCreateWithString(
                core_foundation::base::kCFAllocatorDefault,
                cf_str.as_concrete_TypeRef(),
                ptr::null(),
            )
        }
    }
}
