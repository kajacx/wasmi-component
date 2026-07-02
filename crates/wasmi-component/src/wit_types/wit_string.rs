use std::borrow::Cow;

use anyhow::{Context, Result};
use wasmi::Val;

use crate::{CanonicalWitType, FatPtr, Lift, Lower, MemoryAccess};

pub struct WitString {}

impl CanonicalWitType for WitString {
    type ReturnType = String;

    fn argument_count() -> usize {
        2
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

impl<T: AsStr> Lower for T {
    type WitType = WitString;

    fn lower(&self, output: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()> {
        let contents = self.as_str();

        let (index, bytes) = memory.allocate(contents.len())?;
        bytes.copy_from_slice(contents.as_bytes());

        output[0] = Val::from(index as i32);
        output[1] = Val::from(contents.len() as i32);

        Ok(())
    }
}

// Unfortunately cannot use AsRef<str> directly
pub trait AsStr {
    fn as_str(&self) -> &str;
}

impl<T: AsStr + ?Sized> AsStr for &T {
    fn as_str(&self) -> &str {
        T::as_str(self)
    }
}

impl<T: AsStr + ?Sized> AsStr for &mut T {
    fn as_str(&self) -> &str {
        T::as_str(self)
    }
}

impl AsStr for str {
    fn as_str(&self) -> &str {
        self
    }
}

impl AsStr for String {
    fn as_str(&self) -> &str {
        String::as_str(&self)
    }
}

impl<'a> AsStr for Cow<'a, str> {
    fn as_str(&self) -> &str {
        Cow::as_ref(&self)
    }
}
