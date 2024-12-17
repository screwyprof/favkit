use std::fmt::{Display, Formatter};

use core_foundation::url::{CFURL, CFURLRef};

use crate::{
    finder::{FinderError, Result},
    system::core_foundation::CFRef,
};

pub(crate) struct Url(CFRef<CFURL>);

impl TryFrom<CFURLRef> for Url {
    type Error = FinderError;

    fn try_from(url_ref: CFURLRef) -> Result<Self> {
        CFRef::from_ref(url_ref).map(Self)
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.get_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_foundation::{base::TCFType, string::CFString, url::kCFURLPOSIXPathStyle};

    #[test]
    fn should_return_error_for_null_url() {
        let url_ref: CFURLRef = std::ptr::null_mut();
        assert!(Url::try_from(url_ref).is_err());
    }

    #[test]
    fn should_format_url_using_display_trait() -> Result<()> {
        let path = CFString::new("/Users/user/Documents");
        let valid = CFURL::from_file_system_path(path, kCFURLPOSIXPathStyle, true);
        let url_ref = valid.as_concrete_TypeRef();
        let url = Url::try_from(url_ref)?;
        assert_eq!(url.to_string(), "file:///Users/user/Documents/");
        Ok(())
    }
}
