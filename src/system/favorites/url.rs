use crate::system::core_foundation::CFURLHandle;

pub(crate) type Url = CFURLHandle;

impl From<Url> for String {
    fn from(url: Url) -> Self {
        url.0.get_string().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_foundation::{
        base::TCFType,
        string::CFString,
        url::{CFURL, CFURLRef, kCFURLPOSIXPathStyle},
    };

    #[test]
    fn should_return_none_for_null_url() {
        let url_ref: CFURLRef = std::ptr::null_mut();
        assert!(Url::from_ref(url_ref).is_none());
    }

    #[test]
    fn should_convert_valid_url_to_string() {
        let path = CFString::new("/Users/user/Documents");
        let valid = CFURL::from_file_system_path(path, kCFURLPOSIXPathStyle, true);
        let url_ref = valid.as_concrete_TypeRef();
        let url = Url::from_ref(url_ref).unwrap();
        assert_eq!(String::from(url), "file:///Users/user/Documents/");
    }
}
