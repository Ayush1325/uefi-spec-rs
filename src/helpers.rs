#[inline]
pub(crate) fn null_check_mut<T, E>(ptr: *mut T, err: E) -> Result<(), E> {
    if ptr.is_null() {
        Ok(())
    } else {
        Err(err)
    }
}

#[inline]
pub(crate) fn status_to_result<E>(
    status: r_efi::efi::Status,
    warning: E,
    error: E,
) -> Result<(), E> {
    if status.is_error() {
        Err(error)
    } else if status.is_warning() {
        Err(warning)
    } else {
        Ok(())
    }
}
