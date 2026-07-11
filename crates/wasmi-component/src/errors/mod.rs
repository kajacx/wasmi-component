mod call_error;
mod convert_error;

pub use call_error::*;
pub use convert_error::*;

pub type HostResult<T> = Result<T, wasmi::Error>;
