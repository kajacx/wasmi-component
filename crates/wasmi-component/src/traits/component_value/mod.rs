use std::fmt::Debug;

use crate::lib_structs::LiftReader;
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

    fn byte_align() -> usize;

    fn byte_size() -> usize;

    fn lift<'mem>(reader: &mut impl LiftReader<'mem>) -> ConvertResult<Self::Borrowed<'mem>>;
}
