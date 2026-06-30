use std::fmt::Debug;

use wasmi::WasmResults;

pub trait Lift: Debug {
    type CoreType: WasmResults;

    fn lift(val: Self::CoreType) -> Self;

    fn as_address(_addr: &Self::CoreType) -> Option<i32> {
        None
    }
}

impl Lift for i32 {
    type CoreType = Self;

    fn lift(val: Self::CoreType) -> Self {
        val
    }
}

impl Lift for u32 {
    type CoreType = Self;

    fn lift(val: Self::CoreType) -> Self {
        val
    }
}

impl Lift for f32 {
    type CoreType = Self;

    fn lift(val: Self::CoreType) -> Self {
        val
    }
}

impl Lift for String {
    type CoreType = i32;

    fn lift(val: Self::CoreType) -> Self {
        format!("String from address: {val}")
    }

    fn as_address(addr: &Self::CoreType) -> Option<i32> {
        Some(*addr)
    }
}
