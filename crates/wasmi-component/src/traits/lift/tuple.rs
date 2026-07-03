use anyhow::Result;
use wasmi::{Val, ValType};

use crate::Lift;

impl Lift for () {
    type Borrowed<'a> = Self;

    fn results_count() -> usize {
        0
    }

    fn lift<'a>(_vals: &[Val], _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        Ok(())
    }

    fn into_owned(_val: Self::Borrowed<'_>) -> Self {
        ()
    }

    fn imported_params() -> Vec<ValType> {
        vec![]
    }
}

impl<T0: Lift> Lift for (T0,) {
    type Borrowed<'a> = (T0::Borrowed<'a>,);

    fn lift<'a>(vals: &[Val], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        Ok((T0::lift(vals, memory)?,))
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        (T0::into_owned(val.0),)
    }

    fn imported_params() -> Vec<ValType> {
        T0::imported_params()
    }
}

impl<T0: Lift, T1: Lift> Lift for (T0, T1) {
    type Borrowed<'a> = (T0::Borrowed<'a>, T1::Borrowed<'a>);

    fn lift<'a>(vals: &[Val], memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        // TODO: this will be slow?
        let val0 = T0::lift(&vals[0..T0::imported_params().len()], memory)?;
        let val1 = T1::lift(&vals[T1::imported_params().len()..], memory)?;
        Ok((val0, val1))
    }

    fn into_owned(_val: Self::Borrowed<'_>) -> Self {
        todo!()
    }

    fn imported_params() -> Vec<ValType> {
        let mut params = vec![];
        params.extend(T0::imported_params());
        params.extend(T1::imported_params());
        params
    }
}
