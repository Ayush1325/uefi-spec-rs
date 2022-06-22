//! This module provides constructs for creating and managing Global Pointers.

use crate::errors;
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

    /// SAFETY: This function will only initialize the pointer if it is null.
    /// The return value is a Result containing nothing if it is success. In the case of an
    /// error, it returns the previous pointer.
    pub fn init(&self, ptr: *mut T) -> Result<(), *mut T> {
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
}
