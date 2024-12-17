use core_foundation::url::{CFURL, CFURLRef};

use crate::system::core_foundation::{Raw, Safe};

pub(crate) type RawUrl = Raw<CFURLRef>;
pub(crate) type Url = Safe<CFURL>;

impl From<Url> for String {
    fn from(url: Url) -> Self {
        url.0.get_string().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_foundation::{base::TCFType, string::CFString, url::kCFURLPOSIXPathStyle};

    #[test]
    fn should_return_none_for_null_url() {
        let url_ref: CFURLRef = std::ptr::null_mut();
        let raw = Raw::from(url_ref);
        assert!(Option::<Url>::from(raw).is_none());
    }

    #[test]
    fn should_convert_valid_url_to_string() {
        let path = CFString::new("/Users/user/Documents");
        let valid = CFURL::from_file_system_path(path, kCFURLPOSIXPathStyle, true);
        let url_ref = valid.as_concrete_TypeRef();
        let url = Option::<Url>::from(Raw::from(url_ref)).unwrap();
        assert_eq!(String::from(url), "file:///Users/user/Documents/");
    }
}
