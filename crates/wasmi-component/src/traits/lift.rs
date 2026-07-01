use std::fmt::Debug;

use anyhow::Result;
use wasmi::WasmResults;

pub trait Lift: Debug {
    type CoreType: WasmResults;

    type Borrowed<'a>;

    fn lift<'a>(val: Self::CoreType, memory: &'a [u8]) -> Result<Self::Borrowed<'a>>;

    fn into_owned(val: Self::Borrowed<'_>) -> Self;

    fn as_address(_addr: &Self::CoreType) -> Option<i32> {
        None
    }
}

impl Lift for i32 {
    type CoreType = Self;
    type Borrowed<'a> = Self;

    fn lift<'a>(val: Self::CoreType, _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        Ok(val)
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val
    }
}

impl Lift for u32 {
    type CoreType = Self;
    type Borrowed<'a> = Self;

    fn lift<'a>(val: Self::CoreType, _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        Ok(val)
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val
    }
}

impl Lift for f32 {
    type CoreType = Self;
    type Borrowed<'a> = Self;

    fn lift<'a>(val: Self::CoreType, _memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        Ok(val)
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val
    }
}

impl Lift for String {
    type CoreType = i32;
    type Borrowed<'a> = &'a str;

    fn lift<'a>(val: Self::CoreType, memory: &'a [u8]) -> Result<Self::Borrowed<'a>> {
        let ptr = FatPtr::from_data(memory, val as usize);
        let str_bytes = &memory[ptr.start..(ptr.start + ptr.len)];
        Ok(str::from_utf8(str_bytes)?)
    }

    fn into_owned(val: Self::Borrowed<'_>) -> Self {
        val.to_string()
    }

    // fn with_lifted_value<'a, T>(
    //     val: Self::CoreType,
    //     ctx: impl AsContext,
    //     memory: &'a [u8],
    //     callback: FnOnce(Self::Borrowed<'a>) -> T,
    // ) -> T {
    //     let ptr = FatPtr::from_data(memory, val as usize);
    //     let str_bytes = &memory[ptr.start..(ptr.start + ptr.len)];

    // }

    fn as_address(addr: &Self::CoreType) -> Option<i32> {
        Some(*addr)
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
