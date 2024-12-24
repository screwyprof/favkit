use core_foundation::{base::TCFType, string::CFString};
use favkit::system::favorites::{
    DisplayName as SystemDisplayName,
    errors::{FavoritesError, Result},
};

pub struct DisplayName<'a, T>(Option<&'a T>);

impl<'a, T> DisplayName<'a, T> {
    pub fn from(name: &'a Option<T>) -> Self {
        Self(name.as_ref())
    }
}

impl<'a, T: AsRef<str>> TryFrom<DisplayName<'a, T>> for SystemDisplayName {
    type Error = FavoritesError;

    fn try_from(display_name: DisplayName<'a, T>) -> Result<Self> {
        let name = display_name.0.map(|s| s.as_ref()).unwrap_or_default();
        let cf_string = CFString::new(name);
        Self::try_from(cf_string.as_concrete_TypeRef())
    }
}
