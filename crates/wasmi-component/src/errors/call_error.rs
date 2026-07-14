use std::fmt::Display;

use crate::ConvertError;

#[derive(Debug)]
pub enum CallError {
    /// Error that originated inside of the wasm core module, like a panic or unhandled exception.
    WasmError,

    /// Error caused by a problem when converting a component value to core wasm arguments or vice versa.
    ConvertError(ConvertError),

    /// A resumable trap explicitly created by the host to pause execution so it can be resumed later.
    HostTrap,

    /// Core wasm module has ran out of fuel, execution can be continued later.
    OutOfFuel,
}

pub type CallResult<T> = Result<T, CallError>;

impl From<wasmi::Error> for CallError {
    fn from(value: wasmi::Error) -> Self {
        match value.kind() {
            wasmi::errors::ErrorKind::Fuel(_) => Self::OutOfFuel,
            wasmi::errors::ErrorKind::Host(_) => Self::HostTrap,
            wasmi::errors::ErrorKind::TrapCode(_) => Self::HostTrap, // TODO: ?
            wasmi::errors::ErrorKind::Message(_) => Self::HostTrap,

            _ => Self::WasmError,
        }
    }
}

impl From<ConvertError> for CallError {
    fn from(value: ConvertError) -> Self {
        Self::ConvertError(value)
    }
}

impl Display for CallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WasmError => write!(f, "CallError::WasmError"),
            Self::ConvertError(err) => write!(f, "Call to TODO: name failed: {err}"),
            Self::HostTrap => write!(f, "CallError::HostTrap"),
            Self::OutOfFuel => write!(f, "CallError::OutOfFuel"),
        }
    }
}

impl std::error::Error for CallError {}
