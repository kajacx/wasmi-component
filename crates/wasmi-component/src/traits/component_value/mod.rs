use std::fmt::Debug;

use anyhow::Result;
use wasmi::{Val, ValType};

use crate::View;

mod list;
mod primitive;
mod resource;
mod result;
mod string;
mod tuple;

pub trait ComponentValue: Sized + Debug {
    type Borrowed<'a>: View<Self>;

    fn arg_count() -> usize;

    fn arg_types() -> Vec<ValType>;

    fn lift_args<'a>(vals: &[Val], memory: &'a [u8]) -> Result<Self::Borrowed<'a>>;

    fn byte_align() -> usize;

    fn byte_size() -> usize;

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> Result<Self::Borrowed<'a>>;
}
