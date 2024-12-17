use crate::finder::FinderError;
use core_foundation::base::{TCFType, TCFTypeRef};
use std::{ops::Deref, ptr::NonNull};

#[derive(Clone, Copy)]
pub(crate) struct RawRef<T>(NonNull<T>);

impl<T> From<NonNull<T>> for RawRef<T> {
    fn from(ptr: NonNull<T>) -> Self {
        Self(ptr)
    }
}

impl<T> From<RawRef<T>> for NonNull<T> {
    fn from(raw: RawRef<T>) -> Self {
        raw.0
    }
}

impl<T> From<RawRef<T>> for *mut T {
    fn from(raw: RawRef<T>) -> Self {
        raw.0.as_ptr()
    }
}

impl<T> TryFrom<*mut T> for RawRef<T> {
    type Error = FinderError;

    fn try_from(ptr: *mut T) -> Result<Self, Self::Error> {
        NonNull::new(ptr)
            .map(Self)
            .ok_or(FinderError::NullListHandle)
    }
}

// Generic wrapper for Core Foundation types
pub(crate) struct CFRef<T>(T);

// Core Foundation conversions
impl<T: TCFType> CFRef<T> {
    pub(crate) fn from_ref(raw: T::Ref) -> Result<Self, FinderError>
    where
        T::Ref: TCFTypeRef,
    {
        if raw.as_void_ptr().is_null() {
            return Err(FinderError::NullListHandle);
        }
        Ok(Self(unsafe { T::wrap_under_get_rule(raw) }))
    }
}

impl<T> Deref for CFRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
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
    use crate::finder::Result;

    #[test]
    fn should_return_error_for_null_string() {
        let ptr: CFStringRef = std::ptr::null_mut();
        assert!(CFRef::<CFString>::from_ref(ptr).is_err());
    }

    #[test]
    fn should_convert_valid_string_ref() -> Result<()> {
        let string = CFString::new("test");
        let ptr = string.as_concrete_TypeRef();
        let wrapped = CFRef::<CFString>::from_ref(ptr)?;
        assert_eq!(wrapped.to_string(), "test");
        Ok(())
    }

    #[test]
    fn should_return_error_for_null_array() {
        let ptr: CFArrayRef = std::ptr::null();
        assert!(CFRef::<CFArray<i32>>::from_ref(ptr).is_err());
    }

    #[test]
    fn should_convert_valid_array_ref() -> Result<()> {
        let array = CFArray::from_copyable(&[1, 2, 3]);
        let ptr = array.as_concrete_TypeRef();
        let wrapped = CFRef::<CFArray<i32>>::from_ref(ptr)?;
        assert_eq!(wrapped.len(), 3);
        Ok(())
    }

    #[test]
    fn should_return_error_for_null_url() {
        let ptr: CFURLRef = std::ptr::null_mut();
        assert!(CFRef::<CFURL>::from_ref(ptr).is_err());
    }

    #[test]
    fn should_convert_valid_url_ref() -> Result<()> {
        let path = CFString::new("/test");
        let url = CFURL::from_file_system_path(path, kCFURLPOSIXPathStyle, true);
        let ptr = url.as_concrete_TypeRef();
        let wrapped = CFRef::<CFURL>::from_ref(ptr)?;
        assert_eq!(wrapped.get_string().to_string(), "file:///test/");
        Ok(())
    }

    #[test]
    fn should_convert_between_raw_pointer_and_rawref() -> Result<()> {
        let mut value = 42;
        let ptr = NonNull::new(&mut value as *mut i32).ok_or(FinderError::NullListHandle)?;
        let wrapped = RawRef::from(ptr);
        let unwrapped: *mut i32 = wrapped.into();
        assert_eq!(unwrapped, ptr.as_ptr());
        Ok(())
    }
}
