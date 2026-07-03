use std::borrow::Cow;

use anyhow::Result;
use wasmi::Val;

use crate::{LowerVal, MemoryAccess};

impl<T: AsStr> LowerVal for T {
    type Target = String;

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
