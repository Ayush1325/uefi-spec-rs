//! This module provides constructs for creating and managing Global Pointers.

use crate::{errors, helpers};
use core::sync::atomic::{AtomicPtr, Ordering};

/// It is mostly ment to
/// store SystemTable and SystemHandle.
pub struct GlobalData<T> {
    ptr: AtomicPtr<T>,
}

impl<T> GlobalData<T> {
    /// Initializes GlobalData with internal NULL pointer. This is constant so that it can be used
    /// in statics.
    pub const fn new() -> Self {
        Self {
            ptr: AtomicPtr::new(core::ptr::null_mut()),
        }
    }

    /// SAFETY: This function will only initialize once.
    /// The return value is a Result containing nothing if it is success. In the case of an
    /// error, it returns the previous pointer.
    pub fn init(&self, ptr: *mut T) -> Result<(), *mut T> {
        // Check that the ptr is not null.
        if ptr.is_null() {
            return Err(ptr);
        }

        let r = self.ptr.compare_exchange(
            core::ptr::null_mut(),
            ptr,
            Ordering::SeqCst,
            Ordering::Relaxed,
        );

        match r {
            Ok(_) => Ok(()),
            Err(x) => Err(x),
        }
    }

    /// This return value is a Result mutable reference of internal pointer if it is not null. It
    /// returns a `NullError` if the internal pointer is NULL.
    pub fn get_mut(&mut self) -> Result<&mut *mut T, errors::NullPtrError> {
        let r = self.ptr.get_mut();
        if (*r).is_null() {
            Err(errors::NullPtrError::new("Global Data"))
        } else {
            Ok(r)
        }
    }

    /// The return value is a non-null pointer.
    /// returns a `NullError` if the internal pointer is NULL.
    pub fn load(&self) -> Result<*mut T, errors::NullPtrError> {
        let p = self.ptr.load(Ordering::Relaxed);
        helpers::null_check_mut(p, "Global Data")?;
        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_init_first() {
        let mut temp = 10;
        let tmp_ptr: *mut u32 = &mut temp;

        let global_data = GlobalData::new();
        assert!(global_data.init(tmp_ptr).is_ok());
    }

    #[test]
    fn check_init_second() {
        let mut temp = 10;
        let tmp_ptr: *mut u32 = &mut temp;

        let global_data = GlobalData::new();
        assert!(global_data.init(tmp_ptr).is_ok());
        assert!(global_data.init(tmp_ptr).is_err());
    }

    #[test]
    fn check_without_init() {
        let mut global_data: GlobalData<usize> = GlobalData::new();
        assert!(global_data.get_mut().is_err());
    }

    #[test]
    fn check_init_null() {
        let global_data: GlobalData<usize> = GlobalData::new();
        assert!(global_data.init(core::ptr::null_mut()).is_err())
    }

    #[test]
    fn multiple_refernece() {
        let mut temp = 10;
        let tmp_ptr: *mut u32 = &mut temp;

        let global_data = GlobalData::new();
        assert!(global_data.init(tmp_ptr).is_ok());

        let ref1 = global_data.load().map_err(|_| "help").unwrap();
        let ref2 = global_data.load().map_err(|_| "help").unwrap();

        assert_eq!(ref1, ref2);
    }
}
