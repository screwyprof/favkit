use core_foundation::base::{TCFType, TCFTypeRef};
use std::{ops::Deref, ptr::NonNull};
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

#[cfg(test)]
mod tests {
    use core_foundation::{
        array::{CFArray, CFArrayRef},
        string::{CFString, CFStringRef},
        url::{CFURL, CFURLRef},
    };

    use super::*;

    mod wrapping {
        use super::*;

        #[test]
        fn should_return_error_for_null_string() {
            // Arrange
            let ptr: CFStringRef = std::ptr::null_mut();

            // Act
            let result = CFRef::<CFString>::try_from_ref(ptr);

            // Assert
            assert!(matches!(result.unwrap_err(), Error::NullPointer));
        }

        #[test]
        fn should_return_error_for_null_array() {
            // Arrange
            let ptr: CFArrayRef = std::ptr::null();

            // Act
            let result = CFRef::<CFArray<i32>>::try_from_ref(ptr);

            // Assert
            assert!(matches!(result.unwrap_err(), Error::NullPointer));
        }

        #[test]
        fn should_return_error_for_null_url() {
            // Arrange
            let ptr: CFURLRef = std::ptr::null_mut();

            // Act
            let result = CFRef::<CFURL>::try_from_ref(ptr);

            // Assert
            assert!(matches!(result.unwrap_err(), Error::NullPointer));
        }

        #[test]
        fn should_wrap_string_ref() -> Result<()> {
            // Arrange
            let string = CFString::new("test");
            let ptr = string.as_concrete_TypeRef();

            // Act
            let wrapped = CFRef::<CFString>::try_from_ref(ptr)?;

            // Assert
            assert!(!wrapped.as_concrete_TypeRef().is_null());
            Ok(())
        }

        #[test]
        fn should_wrap_array_ref() -> Result<()> {
            // Arrange
            let array = CFArray::from_copyable(&[1, 2, 3]);
            let ptr = array.as_concrete_TypeRef();

            // Act
            let wrapped = CFRef::<CFArray<i32>>::try_from_ref(ptr)?;

            // Assert
            assert!(!wrapped.as_concrete_TypeRef().is_null());
            Ok(())
        }

        #[test]
        fn should_wrap_url_ref() -> Result<()> {
            // Arrange
            let url = CFURL::from_file_system_path(
                CFString::new("/test"),
                core_foundation::url::kCFURLPOSIXPathStyle,
                true,
            );
            let ptr = url.as_concrete_TypeRef();

            // Act
            let wrapped = CFRef::<CFURL>::try_from_ref(ptr)?;

            // Assert
            assert!(!wrapped.as_concrete_TypeRef().is_null());
            Ok(())
        }
    }

    mod raw_pointer {
        use super::*;

        #[test]
        fn should_convert_raw_pointer() -> Result<()> {
            // Arrange
            let mut value = 42;
            let ptr = NonNull::new(&mut value as *mut i32).unwrap();

            // Act
            let raw = RawRef::new(ptr);
            let back_ptr: *mut i32 = raw.into();

            // Assert
            assert_eq!(back_ptr, ptr.as_ptr());
            Ok(())
        }
    }

    mod dereferencing {
        use super::*;

        #[test]
        fn should_deref_string() -> Result<()> {
            // Arrange
            let string = CFString::new("test");
            let wrapped = CFRef::<CFString>::try_from_ref(string.as_concrete_TypeRef())?;

            // Act
            let derefed: &CFString = &wrapped;

            // Assert
            assert_eq!(derefed.to_string(), "test");
            Ok(())
        }

        #[test]
        fn should_deref_array() -> Result<()> {
            // Arrange
            let array = CFArray::from_copyable(&[1, 2, 3]);
            let wrapped = CFRef::<CFArray<i32>>::try_from_ref(array.as_concrete_TypeRef())?;

            // Act
            let derefed: &CFArray<i32> = &wrapped;

            // Assert
            assert_eq!(derefed.len(), 3);
            Ok(())
        }
    }
}
