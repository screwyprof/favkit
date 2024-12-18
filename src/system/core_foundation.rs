use core_foundation::base::{TCFType, TCFTypeRef};
use std::{fmt, ops::Deref, ptr::NonNull};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("null pointer encountered")]
    NullPointer,
}

pub type Result<T> = std::result::Result<T, Error>;

// RawRef for C-style raw pointers
#[derive(Clone, Copy)]
pub(crate) struct RawRef<T>(NonNull<T>);

impl<T> RawRef<T> {
    pub(crate) fn new(ptr: NonNull<T>) -> Self {
        Self(ptr)
    }
}

impl<T> From<RawRef<T>> for *mut T {
    fn from(raw: RawRef<T>) -> Self {
        raw.0.as_ptr()
    }
}

// CFRef for Core Foundation types
#[derive(Debug)]
pub(crate) struct CFRef<T: TCFType>(T);

impl<T: TCFType> CFRef<T> {
    pub(crate) fn try_from_ref(raw: T::Ref) -> Result<Self>
    where
        T::Ref: TCFTypeRef,
    {
        (!raw.as_void_ptr().is_null())
            .then(|| unsafe { T::wrap_under_get_rule(raw) })
            .map(Self)
            .ok_or(Error::NullPointer)
    }
}

impl<T: TCFType> Deref for CFRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Display implementations for specific Core Foundation types
impl fmt::Display for CFRef<core_foundation::string::CFString> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for CFRef<core_foundation::url::CFURL> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.get_string())
    }
}

#[cfg(test)]
mod tests {
    use core_foundation::{
        array::{CFArray, CFArrayRef},
        string::{CFString, CFStringRef},
        url::{CFURL, CFURLRef, kCFURLPOSIXPathStyle},
    };

    use super::*;

    #[test]
    fn should_return_error_for_null_string() {
        let ptr: CFStringRef = std::ptr::null_mut();
        assert!(matches!(
            CFRef::<CFString>::try_from_ref(ptr).unwrap_err(),
            Error::NullPointer
        ));
    }

    #[test]
    fn should_convert_valid_string_ref() -> Result<()> {
        let string = CFString::new("test");
        let ptr = string.as_concrete_TypeRef();
        let wrapped = CFRef::<CFString>::try_from_ref(ptr)?;
        assert_eq!(wrapped.to_string(), "test");
        Ok(())
    }

    #[test]
    fn should_return_error_for_null_array() {
        let ptr: CFArrayRef = std::ptr::null();
        assert!(matches!(
            CFRef::<CFArray<i32>>::try_from_ref(ptr).unwrap_err(),
            Error::NullPointer
        ));
    }

    #[test]
    fn should_convert_valid_array_ref() -> Result<()> {
        let array = CFArray::from_copyable(&[1, 2, 3]);
        let ptr = array.as_concrete_TypeRef();
        let wrapped = CFRef::<CFArray<i32>>::try_from_ref(ptr)?;
        assert_eq!(wrapped.len(), 3);
        Ok(())
    }

    #[test]
    fn should_return_error_for_null_url() {
        let ptr: CFURLRef = std::ptr::null_mut();
        assert!(matches!(
            CFRef::<CFURL>::try_from_ref(ptr).unwrap_err(),
            Error::NullPointer
        ));
    }

    #[test]
    fn should_convert_valid_url_ref() -> Result<()> {
        let path = CFString::new("/test");
        let url = CFURL::from_file_system_path(path, kCFURLPOSIXPathStyle, true);
        let ptr = url.as_concrete_TypeRef();
        let wrapped = CFRef::<CFURL>::try_from_ref(ptr)?;
        assert_eq!(wrapped.to_string(), "file:///test/");
        Ok(())
    }

    #[test]
    fn should_convert_raw_pointer() -> Result<()> {
        let mut value = 42;
        let ptr = NonNull::new(&mut value as *mut i32).unwrap();
        let raw = RawRef::new(ptr);
        let back_ptr: *mut i32 = raw.into();
        assert_eq!(back_ptr, ptr.as_ptr());
        Ok(())
    }
}
