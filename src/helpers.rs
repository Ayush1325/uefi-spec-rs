use crate::errors::StatusError;

#[inline]
pub(crate) fn null_check_mut<T, E>(ptr: *mut T, err: E) -> Result<(), E> {
    if ptr.is_null() {
        Err(err)
    } else {
        Ok(())
    }
}

#[inline]
pub(crate) fn status_to_result(status: r_efi::efi::Status) -> Result<(), StatusError> {
    if status.is_error() {
        Err(StatusError::UefiError(status.as_usize()))
    } else if status.is_warning() {
        Err(StatusError::UefiWarning(status.as_usize()))
    } else {
        Ok(())
    }
}
