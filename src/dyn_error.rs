use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[repr(transparent)]
pub struct GenericError(String);

impl Display for GenericError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for GenericError {}

pub trait ToGenericError {
    fn to_error(self) -> GenericError;
    fn wrap_error(self) -> DynResult<()> where Self: Sized {
        Err(Box::new(self.to_error()))
    }
}

impl<T: Into<String>> ToGenericError for T {
    fn to_error(self) -> GenericError {
        GenericError(self.into())
    }
}

pub type DynResult<T> = Result<T, Box<dyn Error>>;