use anyhow::Result;
use wasmi::{Val, ValType};

mod primitive;
mod string;
mod tuple;

pub trait Lift {
    type Borrowed<'a>;

    fn results_count() -> usize {
        1
    }

    fn lift<'a>(vals: &[Val], memory: &'a [u8]) -> Result<Self::Borrowed<'a>>;

    fn into_owned(val: Self::Borrowed<'_>) -> Self;

    fn imported_params() -> Vec<ValType>;
}
