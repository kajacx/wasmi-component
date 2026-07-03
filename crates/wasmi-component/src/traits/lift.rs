use std::fmt::Debug;

use anyhow::{Context, Result};
use wasmi::Val;

pub trait Lift: Debug {
    type Borrowed<'a>;

    fn results_count() -> usize {
        1
    }

    fn lift<'a>(val: Val, memory: &'a [u8]) -> Result<Self::Borrowed<'a>>;

    fn into_owned(val: Self::Borrowed<'_>) -> Self;
}

impl Lift for i32 {
    type Borrowed<'a> = Self;

    fn lift<'a>(val: Val, _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        val.i32().context("Lifting i32")
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val
    }
}

impl Lift for u32 {
    type Borrowed<'a> = Self;

    fn lift<'a>(val: Val, _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        val.i32().map(|val| val as u32).context("Lifting u32")
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val
    }
}

impl Lift for f32 {
    type Borrowed<'a> = Self;

    fn lift<'a>(val: Val, _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        val.f32().map(|val| val.to_float()).context("Lifting f32")
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val
    }
}

impl Lift for () {
    type Borrowed<'a> = Self;

    fn results_count() -> usize {
        0
    }

    fn lift<'a>(_val: Val, _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        Ok(())
    }

    fn into_owned(_val: Self::Borrowed<'_>) -> Self {
        ()
    }
}

impl<T0: Lift> Lift for (T0,) {
    type Borrowed<'a> = T0::Borrowed<'a>;

    fn lift<'a>(val: Val, memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        T0::lift(val, memory)
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        (T0::into_owned(val),)
    }
}

impl<T0: Lift, T1: Lift> Lift for (T0, T1) {
    type Borrowed<'a> = (T0::Borrowed<'a>, T1::Borrowed<'a>);

    fn lift<'a>(_val: Val, _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        todo!()
    }

    fn into_owned(_val: Self::Borrowed<'_>) -> Self {
        todo!()
    }
}
