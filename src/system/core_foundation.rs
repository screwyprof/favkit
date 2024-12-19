//! Core Foundation type wrappers and utilities.
//!
//! This module provides safe wrappers around Core Foundation types and raw pointers.
//! It ensures proper memory management and null pointer handling.
//!
//! # Examples
//!
//! Working with Core Foundation types using `CFRef`:
//! ```no_run
//! use core_foundation::{
//!     base::TCFType,
//!     string::{CFString, CFStringRef}
//! };
//! # use favkit::system::core_foundation::{CFRef, Error};
//!
//! let cf_str = CFString::new("example");
//! let ptr = cf_str.as_concrete_TypeRef();
//! let wrapped = CFRef::<CFString>::try_from_ref(ptr)?;
//! assert_eq!(wrapped.to_string(), "example");
//! # Ok::<(), Error>(())
//! ```
//!
//! Working with raw pointers using `RawRef`:
//! ```
//! use std::ptr::NonNull;
//! # use favkit::system::core_foundation::{RawRef, Error};
//!
//! // Creating from NonNull (when you know the pointer is valid)
//! let mut value = 42;
//! let ptr = NonNull::new(&mut value as *mut i32).unwrap();
//! let raw = RawRef::new(ptr);
//!
//! // Creating from raw pointer (with null check)
//! let ptr = &mut value as *mut i32;
//! let raw = RawRef::try_from(ptr)?;
//!
//! // Converting back to raw pointer
//! let back_ptr: *mut i32 = raw.into();
//! unsafe {
//!     assert_eq!(*back_ptr, 42);
//! }
//! # Ok::<(), Error>(())
//! ```

use core_foundation::base::{TCFType, TCFTypeRef};
use std::{ops::Deref, ptr::NonNull};
use thiserror::Error;

/// Errors that can occur when working with Core Foundation types.
#[derive(Debug, Error)]
pub enum Error {
    /// Returned when a null pointer is encountered where a valid pointer was expected.
    #[error("null pointer encountered")]
    NullPointer,
}

/// Specialized Result type for Core Foundation operations.
pub type Result<T> = std::result::Result<T, Error>;

/// A safe wrapper around non-null raw pointers.
///
/// This type guarantees that the pointer is non-null and properly aligned.
/// It's primarily used for passing pointers to and from C APIs.
///
/// # Examples
///
/// ```
/// use std::ptr::NonNull;
/// # use favkit::system::core_foundation::{RawRef, Error};
///
/// // Safe creation from NonNull
/// let mut value = 42;
/// let ptr = NonNull::new(&mut value as *mut i32).unwrap();
/// let raw = RawRef::new(ptr);
///
/// // Fallible creation from raw pointer
/// let ptr = &mut value as *mut i32;
/// let raw = RawRef::try_from(ptr)?;
///
/// // Converting back to raw pointer
/// let back_ptr: *mut i32 = raw.into();
/// unsafe {
///     assert_eq!(*back_ptr, 42);
/// }
/// # Ok::<(), Error>(())
/// ```
#[derive(Clone, Copy)]
pub struct RawRef<T>(NonNull<T>);

impl<T> RawRef<T> {
    /// Creates a new `RawRef` from a non-null pointer.
    pub fn new(ptr: NonNull<T>) -> Self {
        Self(ptr)
    }
}

/// Attempts to create a `RawRef` from a raw pointer.
///
/// Returns `Error::NullPointer` if the pointer is null.
///
/// # Examples
///
/// ```
/// # use favkit::system::core_foundation::{RawRef, Error};
/// let mut value = 42;
/// let ptr = &mut value as *mut i32;
/// let raw = RawRef::try_from(ptr)?;
/// # Ok::<(), Error>(())
/// ```
impl<T> TryFrom<*mut T> for RawRef<T> {
    type Error = Error;

    fn try_from(ptr: *mut T) -> Result<Self> {
        NonNull::new(ptr).map(Self::new).ok_or(Error::NullPointer)
    }
}

/// Converts a `RawRef` back into a raw pointer.
///
/// The resulting pointer is guaranteed to be non-null.
///
/// # Examples
///
/// ```
/// # use std::ptr::NonNull;
/// # use favkit::system::core_foundation::RawRef;
/// let mut value = 42;
/// let ptr = NonNull::new(&mut value as *mut i32).unwrap();
/// let raw = RawRef::new(ptr);
/// let back_ptr: *mut i32 = raw.into();
/// unsafe {
///     assert_eq!(*back_ptr, 42);
/// }
/// ```
impl<T> From<RawRef<T>> for *mut T {
    fn from(raw: RawRef<T>) -> Self {
        raw.0.as_ptr()
    }
}

/// A safe wrapper around Core Foundation types.
///
/// This type ensures proper reference counting and memory management for Core Foundation objects.
/// It automatically releases the wrapped object when dropped.
///
/// # Type Parameters
///
/// * `T` - A Core Foundation type that implements `TCFType`
///
/// # Examples
///
/// ```no_run
/// use core_foundation::{
///     base::TCFType,
///     array::{CFArray, CFArrayRef}
/// };
/// # use favkit::system::core_foundation::{CFRef, Error};
///
/// let array = CFArray::from_copyable(&[1, 2, 3]);
/// let ptr = array.as_concrete_TypeRef();
/// let wrapped = CFRef::<CFArray<i32>>::try_from_ref(ptr)?;
/// assert_eq!(wrapped.len(), 3);
/// # Ok::<(), Error>(())
/// ```
#[derive(Debug, Clone)]
pub struct CFRef<T: TCFType>(T);

impl<T: TCFType> CFRef<T> {
    /// Attempts to create a `CFRef` from a Core Foundation type reference.
    ///
    /// Returns `Error::NullPointer` if the reference is null.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use core_foundation::{
    ///     base::TCFType,
    ///     string::CFString,
    ///     url::{CFURL, CFURLRef, kCFURLPOSIXPathStyle}
    /// };
    /// # use favkit::system::core_foundation::{CFRef, Error};
    ///
    /// let url = CFURL::from_file_system_path(
    ///     CFString::new("/test"),
    ///     kCFURLPOSIXPathStyle,
    ///     true
    /// );
    /// let ptr = url.as_concrete_TypeRef();
    /// let wrapped = CFRef::<CFURL>::try_from_ref(ptr)?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn try_from_ref(raw: T::Ref) -> Result<Self>
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
