use core_foundation::{
    base::TCFType,
    string::CFString,
    url::{CFURL, kCFURLPOSIXPathStyle},
};
use favkit::system::favorites::{
    Url as SystemUrl,
    errors::{FavoritesError, Result},
};

pub struct Url<T: AsRef<str>>(T);

impl<T: AsRef<str>> From<T> for Url<T> {
    fn from(path: T) -> Self {
        Self(path)
    }
}

impl<T: AsRef<str>> TryFrom<Url<T>> for SystemUrl {
    type Error = FavoritesError;

    fn try_from(url: Url<T>) -> Result<Self> {
        let is_dir = url.0.as_ref().ends_with('/');
        let file_path = CFString::new(url.0.as_ref());
        let url_cf = CFURL::from_file_system_path(file_path, kCFURLPOSIXPathStyle, is_dir);
        Self::try_from(url_cf.as_concrete_TypeRef())
    }
}
