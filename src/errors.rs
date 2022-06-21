//! This module contains varous error types used in this crate

#[derive(Debug, PartialEq, Eq)]
pub enum StatusError {
    NullPtr(&'static str),
    UEFIWarning(usize),
    UEFIError(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub struct NullError;
