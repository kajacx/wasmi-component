pub use anyhow;
pub use wasmi;

mod structs;
mod traits;

pub use structs::*;
pub use traits::*;

pub type HostResult<T> = Result<T, wasmi::Error>;

pub fn anyhow_result_to_wasmi<T>(result: anyhow::Result<T>) -> Result<T, wasmi::Error> {
    result.map_err(|error| wasmi::Error::new(error.to_string()))
}
