use std::fmt::Debug;

use anyhow::{Context, Result};
use wasmi::Val;

pub trait Lift: Debug {
    type Borrowed<'a>;

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

impl Lift for String {
    type Borrowed<'a> = &'a str;

    fn lift<'a>(val: Val, memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        let ptr = FatPtr::from_data(memory, val.i32().context("Lifting String")? as usize);
        let str_bytes = &memory[ptr.start..(ptr.start + ptr.len)];
        Ok(str::from_utf8(str_bytes)?)
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val.to_string()
    }
}

struct FatPtr {
    start: usize,
    len: usize,
}

impl FatPtr {
    pub fn from_data(data: &[u8], addr: usize) -> Self {
        let start = u32::from_le_bytes(data[addr..(addr + 4)].try_into().unwrap()) as usize;
        let len = u32::from_le_bytes(data[(addr + 4)..(addr + 8)].try_into().unwrap()) as usize;
        Self { start, len }
    }
}
