use std::fmt::Display;

use crate::ConvertError;

#[derive(Debug, Clone)]
pub enum CallError {
    WasmError,
    ConvertError,
    HostTrap,
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
    fn from(_value: ConvertError) -> Self {
        Self::ConvertError // TODO:
    }
}

impl Display for CallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WasmError => write!(f, "CallError::WasmError"),
            Self::ConvertError => write!(f, "CallError::ConvertError"),
            Self::HostTrap => write!(f, "CallError::HostTrap"),
            Self::OutOfFuel => write!(f, "CallError::OutOfFuel"),
        }
    }
}

impl std::error::Error for CallError {}
