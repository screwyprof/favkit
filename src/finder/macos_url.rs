use core_foundation::base::{TCFType, kCFAllocatorDefault};
use core_foundation::string::CFString;
use core_foundation::url::{CFURLRef, CFURL, kCFURLPOSIXPathStyle};
use std::path::PathBuf;
use std::ptr;

use super::target::{Target, TargetLocation};

/// Convert a CFURLRef to a Target
/// 
/// # Safety
/// 
/// The caller must ensure that:
/// - The `url` parameter is either null or a valid CFURLRef
/// - The CFURLRef must remain valid for the duration of this function call
pub unsafe fn url_to_target(url: CFURLRef) -> Target {
    if url.is_null() {
        return Target::Home(TargetLocation::Path(dirs::home_dir().unwrap_or_default()));
    }

    let url = CFURL::wrap_under_get_rule(url);
    let cf_string = url.get_string();
    let url_string = cf_string.to_string();

    // Special handling for AirDrop
    if url_string.contains("nwnode://domain-AirDrop") {
        return Target::AirDrop(TargetLocation::Url(url_string));
    }

    // Handle regular file paths
    let path = url.to_path().unwrap_or_default();
    let path_str = path.to_string_lossy();
    let path_str = path_str.strip_prefix("file://").unwrap_or(&path_str);
    let path = PathBuf::from(path_str);

    // Check against known locations
    if let Some(home_dir) = dirs::home_dir() {
        if path == home_dir {
            return Target::Home(TargetLocation::Path(path));
        }
        if path == home_dir.join("Desktop") {
            return Target::Desktop(TargetLocation::Path(path));
        }
        if path == home_dir.join("Documents") {
            return Target::Documents(TargetLocation::Path(path));
        }
        if path == home_dir.join("Downloads") {
            return Target::Downloads(TargetLocation::Path(path));
        }
    }

    if path == PathBuf::from("/Applications") {
        return Target::Applications(TargetLocation::Path(path));
    }

    Target::CustomPath(TargetLocation::Path(path))
}

/// Convert a Target to a CFURLRef
/// 
/// # Safety
/// 
/// This function is unsafe because it creates a Core Foundation URL that must be properly released.
pub fn target_to_url(target: &Target) -> CFURLRef {
    unsafe {
        match target.location() {
            TargetLocation::Url(url) => {
                let cf_str = CFString::new(url);
                core_foundation::url::CFURLCreateWithString(
                    kCFAllocatorDefault,
                    cf_str.as_concrete_TypeRef(),
                    ptr::null(),
                )
            }
            TargetLocation::Path(path) => {
                let cf_str = CFString::new(&path.to_string_lossy());
                core_foundation::url::CFURLCreateWithFileSystemPath(
                    kCFAllocatorDefault,
                    cf_str.as_concrete_TypeRef(),
                    kCFURLPOSIXPathStyle,
                    if path.is_dir() { 1 } else { 0 },
                )
            }
        }
    }
}
