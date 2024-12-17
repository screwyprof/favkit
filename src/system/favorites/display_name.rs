use std::fmt::{Display, Formatter};

use core_foundation::string::{CFString, CFStringRef};

use crate::{
    finder::{FinderError, Result},
    system::core_foundation::CFRef,
};

pub(crate) struct DisplayName(CFRef<CFString>);

impl TryFrom<CFStringRef> for DisplayName {
    type Error = FinderError;

    fn try_from(string_ref: CFStringRef) -> Result<Self> {
        CFRef::from_ref(string_ref).map(Self)
    }
}

impl Display for DisplayName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_foundation::base::TCFType;

    #[test]
    fn should_return_error_for_null_string() {
        let string_ref: CFStringRef = std::ptr::null_mut();
        assert!(DisplayName::try_from(string_ref).is_err());
    }

    #[test]
    fn should_format_display_name_using_display_trait() -> Result<()> {
        let valid = CFString::new("Documents");
        let string_ref = valid.as_concrete_TypeRef();
        let display_name = DisplayName::try_from(string_ref)?;
        assert_eq!(display_name.to_string(), "Documents");
        Ok(())
    }
}
