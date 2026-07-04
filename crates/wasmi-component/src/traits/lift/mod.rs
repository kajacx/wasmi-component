use anyhow::Result;
use wasmi::Val;

use crate::FlatArgs;

mod primitive;
mod string;
mod tuple;

pub trait Lift: FlatArgs {
    type Borrowed<'a>;

    fn lift_args<'a>(vals: &[Val], memory: &'a [u8]) -> Result<Self::Borrowed<'a>>;

    fn lift_bytes<'a>(bytes: &[u8], memory: &'a [u8]) -> Result<Self::Borrowed<'a>>;

    fn into_owned(val: Self::Borrowed<'_>) -> Self;
}
