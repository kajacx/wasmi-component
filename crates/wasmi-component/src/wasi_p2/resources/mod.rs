mod descriptor;
mod error;
mod input_stream;
mod output_stream;
mod pollable;
mod terminal_input;
mod terminal_output;

pub use descriptor::*;
pub use error::*;
pub use input_stream::*;
pub use output_stream::*;
pub use pollable::*;
pub use terminal_input::*;
pub use terminal_output::*;

use anyhow::Result;
use wasmi::ValType;

use crate::{CompValue, LowerVal, Own};

#[allow(unused)]
#[derive(Debug)]
pub enum StreamError {
    LastOperationFailed(Own<ErrorResource>),
    Closed,
}

impl CompValue for StreamError {
    type Borrowed<'a> = Self;

    fn arg_count() -> usize {
        1 + Own::<ErrorResource>::arg_count()
    }

    fn arg_types() -> Vec<ValType> {
        let mut args = vec![ValType::I32];
        args.extend(Own::<ErrorResource>::arg_types());
        args
    }

    fn lift_args<'a>(_vals: &[wasmi::Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        todo!()
    }

    fn byte_align() -> usize {
        Own::<ErrorResource>::byte_align()
    }

    fn byte_size() -> usize {
        Own::<ErrorResource>::byte_align()
    }

    fn lift_bytes<'a>(_bytes: &[u8], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        todo!()
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val
    }
}

#[allow(unused)]
impl LowerVal<StreamError> for StreamError {
    fn lower_args(
        &self,
        args: &mut [wasmi::Val],
        memory: &mut impl crate::MemoryAccess,
    ) -> Result<()> {
        todo!()
    }

    fn lower_bytes(
        &self,
        range: std::ops::Range<usize>,
        memory: &mut impl crate::MemoryAccess,
    ) -> Result<()> {
        todo!()
    }
}
