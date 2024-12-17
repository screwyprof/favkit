use std::fmt;

use crate::system::core_foundation::CFRef;
use core_foundation::url::CFURL;

pub(crate) type Url = CFRef<CFURL>;

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_ref() {
            Some(url) => write!(f, "{}", url.get_string()),
            None => write!(f, "<none>"),
        }
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

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn should_return_none_for_null_url() -> Result<()> {
        let url_ref: CFURLRef = std::ptr::null_mut();
        assert!(Url::from_ref(url_ref).as_ref().is_none());
        Ok(())
    }

    #[test]
    fn should_format_url_using_display_trait() -> Result<()> {
        let path = CFString::new("/Users/user/Documents");
        let valid = CFURL::from_file_system_path(path, kCFURLPOSIXPathStyle, true);
        let url_ref = valid.as_concrete_TypeRef();
        let url = Url::from_ref(url_ref);
        assert!(url.as_ref().is_some());
        assert_eq!(format!("{}", url), "file:///Users/user/Documents/");
        Ok(())
    }
}
