use std::error::Error;

#[derive(Debug)]
pub struct ConvertError {
    pub message: String,
    pub cause: Option<Box<dyn Error>>,
}

pub type ConvertResult<T> = Result<T, ConvertError>;

impl ConvertError {
    pub fn new(message: String) -> Self {
        Self {
            message,
            cause: None,
        }
    }

    pub fn with_cause(message: String, cause: Box<dyn Error>) -> Self {
        Self {
            message,
            cause: Some(cause),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConvertErrorType {
    Lift,
    Lower,
}

impl From<ConvertError> for wasmi::Error {
    fn from(value: ConvertError) -> Self {
        wasmi::Error::new(format!("{}", value))
    }
}

impl std::fmt::Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "conversion failed because \"{}\"", self.message)
    }
}

impl std::error::Error for ConvertError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_deref()
    }
}
