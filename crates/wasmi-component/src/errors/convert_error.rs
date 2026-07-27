use std::error::Error;

#[derive(Debug)]
pub struct ConvertError {
    pub message: String,
    pub additional: Option<String>,
    pub cause: Option<Box<dyn Error + Send + Sync + 'static>>,
}

pub type ConvertResult<T> = Result<T, ConvertError>;

impl ConvertError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            additional: None,
            cause: None,
        }
    }

    pub fn with_additional(mut self, additional: impl Into<String>) -> Self {
        self.additional = Some(additional.into());
        self
    }

    pub fn with_cause(mut self, cause: Box<dyn Error + Send + Sync>) -> Self {
        self.cause = Some(cause);
        self
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
        self.cause.as_ref().map(|e| &**e as _)
    }
}
