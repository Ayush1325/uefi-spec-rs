//! This module contains various error types used in this crate

#[derive(PartialEq, Eq)]
pub enum StatusNullError {
    NullPtrError(&'static str),
    UefiWarning(usize),
    UefiError(usize),
}

impl From<StatusError> for StatusNullError {
    fn from(x: StatusError) -> Self {
        match x {
            StatusError::UefiError(e) => Self::UefiError(e),
            StatusError::UefiWarning(w) => Self::UefiWarning(w),
        }
    }
}

impl From<NullPtrError> for StatusNullError {
    fn from(x: NullPtrError) -> Self {
        Self::NullPtrError(x.0)
    }
}

#[derive(PartialEq, Eq)]
pub enum StatusError {
    UefiWarning(usize),
    UefiError(usize),
}

#[derive(PartialEq, Eq)]
pub struct NullPtrError(&'static str);

impl NullPtrError {
    pub fn new(ptr_name: &'static str) -> Self {
        Self(ptr_name)
    }
}
