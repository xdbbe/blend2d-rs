use core::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("error")
    }
}

#[inline]
pub fn err_to_result(code: u32) -> Result<(), Error> {
    match code {
        0 => Ok(()),
        _ => Err(error_from_errcode(code)),
    }
}

#[cold]
fn error_from_errcode(_code: u32) -> Error {
    // TODO
    Error
}
