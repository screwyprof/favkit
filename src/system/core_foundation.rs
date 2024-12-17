use core_foundation::base::{TCFType, TCFTypeRef};

// Generic reference-counted wrapper for Core Foundation types
pub(crate) struct CFRef<T>(pub(crate) T);

// Generic wrapper for raw pointer types
#[derive(Clone, Copy)]
pub(crate) struct RawRef<T>(pub(crate) *mut T);

// Core Foundation conversions
impl<T: TCFType> CFRef<T> {
    pub(crate) fn from_ref(raw: T::Ref) -> Option<Self>
    where
        T::Ref: TCFTypeRef,
    {
        (!raw.as_void_ptr().is_null()).then(|| unsafe { Self(T::wrap_under_get_rule(raw)) })
    }
}

// Raw pointer conversions
impl<T> RawRef<T> {
    pub(crate) fn from_ref(raw: *mut T) -> Option<Self> {
        (!raw.is_null()).then_some(Self(raw))
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

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn should_return_none_for_null_string() -> Result<()> {
        let ptr: CFStringRef = std::ptr::null_mut();
        assert!(CFRef::<CFString>::from_ref(ptr).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_string() -> Result<()> {
        let string = CFString::new("test");
        let ptr = string.as_concrete_TypeRef();
        let wrapped: CFRef<CFString> = CFRef::from_ref(ptr).ok_or("Failed to create CFRef")?;
        assert_eq!(wrapped.0.to_string(), "test");
        Ok(())
    }

    #[test]
    fn should_return_none_for_null_array() -> Result<()> {
        let ptr: CFArrayRef = std::ptr::null();
        assert!(CFRef::<CFArray<i32>>::from_ref(ptr).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_array() -> Result<()> {
        let array = CFArray::from_copyable(&[1, 2, 3]);
        let ptr = array.as_concrete_TypeRef();
        let wrapped: CFRef<CFArray<i32>> = CFRef::from_ref(ptr).ok_or("Failed to create CFRef")?;
        assert_eq!(wrapped.0.len(), 3);
        Ok(())
    }

    #[test]
    fn should_return_none_for_null_url() -> Result<()> {
        let ptr: CFURLRef = std::ptr::null_mut();
        assert!(CFRef::<CFURL>::from_ref(ptr).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_url() -> Result<()> {
        let path = CFString::new("/test");
        let url = CFURL::from_file_system_path(path, kCFURLPOSIXPathStyle, true);
        let ptr = url.as_concrete_TypeRef();
        let wrapped: CFRef<CFURL> = CFRef::from_ref(ptr).ok_or("Failed to create CFRef")?;
        assert_eq!(wrapped.0.get_string().to_string(), "file:///test/");
        Ok(())
    }

    #[test]
    fn should_return_none_for_null_raw_pointer() -> Result<()> {
        let ptr: *mut i32 = std::ptr::null_mut();
        assert!(RawRef::<i32>::from_ref(ptr).is_none());
        Ok(())
    }

    #[test]
    fn should_wrap_valid_raw_pointer() -> Result<()> {
        let mut value = 42;
        let ptr = &mut value as *mut i32;
        let wrapped = RawRef::<i32>::from_ref(ptr).ok_or("Failed to create RawRef")?;
        assert_eq!(wrapped.0, ptr);
        Ok(())
    }
}
