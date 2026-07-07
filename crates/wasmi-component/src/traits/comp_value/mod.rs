use std::fmt::Debug;

use anyhow::Result;
use wasmi::{Val, ValType};

mod list;
mod primitive;
mod resource;
mod result;
mod string;
mod tuple;

pub trait CompValue: Sized + Debug {
    type Borrowed<'a>: IntoOwned<Self>;

    fn arg_count() -> usize;

    fn arg_types() -> Vec<ValType>;

    fn lift_args<'a>(vals: &[Val], memory: &'a [u8]) -> Result<Self::Borrowed<'a>>;

    fn byte_align() -> usize;

    fn byte_size() -> usize;

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> Result<Self::Borrowed<'a>>;
}

pub trait IntoOwned<T> {
    fn into_owned(self) -> T;
}
