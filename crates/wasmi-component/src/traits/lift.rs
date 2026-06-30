use std::fmt::Debug;

use wasmi::{AsContext, Memory, WasmResults};

pub trait Lift: Debug {
    type CoreType: WasmResults;

    fn lift(val: Self::CoreType, store: impl AsContext, memory: &Memory) -> Self;

    fn as_address(_addr: &Self::CoreType) -> Option<i32> {
        None
    }
}

impl Lift for i32 {
    type CoreType = Self;

    fn lift(val: Self::CoreType, _: impl AsContext, _: &Memory) -> Self {
        val
    }
}

impl Lift for u32 {
    type CoreType = Self;

    fn lift(val: Self::CoreType, _: impl AsContext, _: &Memory) -> Self {
        val
    }
}

impl Lift for f32 {
    type CoreType = Self;

    fn lift(val: Self::CoreType, _: impl AsContext, _: &Memory) -> Self {
        val
    }
}

impl Lift for String {
    type CoreType = i32;

    fn lift(val: Self::CoreType, ctx: impl AsContext, memory: &Memory) -> Self {
        let data = memory.data(ctx.as_context());
        let ptr = FatPtr::from_data(data, val as usize);

        let str_bytes = &data[ptr.start..(ptr.start + ptr.len)];
        str::from_utf8(str_bytes).unwrap().to_string()
    }

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
