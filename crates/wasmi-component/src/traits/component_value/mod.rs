use std::fmt::Debug;

use crate::lib_structs::WasmValue;
use crate::{ConvertResult, Lift, ValueType};

mod list;
mod option;
mod primitive;
mod string;
mod tuple;

pub trait ComponentValue: Sized + Debug {
    type Borrowed<'a>: Lift<Self>;

    fn value_type() -> ValueType;

    fn arg_count() -> usize;

    fn lift_args<'a>(args: &[WasmValue], memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>>;

    fn byte_align() -> usize;

    fn byte_size() -> usize;

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>>;
}
