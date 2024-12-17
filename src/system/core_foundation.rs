use core_foundation::base::{TCFType, TCFTypeRef};
// Generic wrapper for raw pointer types
#[derive(Clone, Copy)]
pub(crate) struct RawRef<T>(Option<*mut T>);

// From raw pointer to RawRef
impl<T> From<*mut T> for RawRef<T> {
    fn from(ptr: *mut T) -> Self {
        Self((!ptr.is_null()).then_some(ptr))
    }
}

// From RawRef to raw pointer
impl<T> From<RawRef<T>> for *mut T {
    fn from(raw: RawRef<T>) -> Self {
        raw.0.unwrap_or(std::ptr::null_mut())
    }
}

// Generic wrapper for Core Foundation types
pub(crate) struct CFRef<T>(Option<T>);

// Core Foundation conversions
impl<T: TCFType> CFRef<T> {
    pub(crate) fn from_ref(raw: T::Ref) -> Self
    where
        T::Ref: TCFTypeRef,
    {
        Self((!raw.as_void_ptr().is_null()).then(|| unsafe { T::wrap_under_get_rule(raw) }))
    }

    pub(crate) fn as_ref(&self) -> Option<&T> {
        self.0.as_ref()
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
        assert!(CFRef::<CFString>::from_ref(ptr).as_ref().is_none());
        Ok(())
    }

    #[test]
    fn should_convert_valid_string_ref() -> Result<()> {
        let string = CFString::new("test");
        let ptr = string.as_concrete_TypeRef();
        let wrapped = CFRef::<CFString>::from_ref(ptr);
        assert_eq!(wrapped.as_ref().unwrap().to_string(), "test");
        Ok(())
    }

    #[test]
    fn should_return_none_for_null_array() -> Result<()> {
        let ptr: CFArrayRef = std::ptr::null();
        assert!(CFRef::<CFArray<i32>>::from_ref(ptr).as_ref().is_none());
        Ok(())
    }

    #[test]
    fn should_convert_valid_array_ref() -> Result<()> {
        let array = CFArray::from_copyable(&[1, 2, 3]);
        let ptr = array.as_concrete_TypeRef();
        let wrapped = CFRef::<CFArray<i32>>::from_ref(ptr);
        assert_eq!(wrapped.as_ref().unwrap().len(), 3);
        Ok(())
    }

    #[test]
    fn should_return_none_for_null_url() -> Result<()> {
        let ptr: CFURLRef = std::ptr::null_mut();
        assert!(CFRef::<CFURL>::from_ref(ptr).as_ref().is_none());
        Ok(())
    }

    #[test]
    fn should_convert_valid_url_ref() -> Result<()> {
        let path = CFString::new("/test");
        let url = CFURL::from_file_system_path(path, kCFURLPOSIXPathStyle, true);
        let ptr = url.as_concrete_TypeRef();
        let wrapped = CFRef::<CFURL>::from_ref(ptr);
        assert_eq!(
            wrapped.as_ref().unwrap().get_string().to_string(),
            "file:///test/"
        );
        Ok(())
    }

    #[test]
    fn should_convert_rawref_to_raw_pointer() -> Result<()> {
        let mut value = 42;
        let ptr = &mut value as *mut i32;
        let wrapped = RawRef::from(ptr);
        let unwrapped: *mut i32 = wrapped.into();
        assert_eq!(unwrapped, ptr);
        Ok(())
    }
}
