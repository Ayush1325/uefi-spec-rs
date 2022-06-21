#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NullPtr(&'static str),
    UEFIWarning(usize),
    UEFIError(usize),
}
