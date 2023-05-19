use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct InvalidActionError(String);
pub type ActionResult = Result<(), InvalidActionError>;

impl Display for InvalidActionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl InvalidActionError {
    pub fn new(message: String) -> Self {
        Self(message)
    }
    pub fn into_err(self) -> ActionResult {
        Err(self)
    }
}

impl Error for InvalidActionError {}

impl From<&str> for InvalidActionError {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
