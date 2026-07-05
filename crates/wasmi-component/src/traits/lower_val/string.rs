use std::{borrow::Cow, ops::Range};

use anyhow::Result;
use wasmi::Val;

use crate::{CompValue, LowerVal, MemoryAccess};

impl<T: AsStr> LowerVal for T {
    type Target = String;

    fn lower_args(&self, output: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(output.len(), Self::Target::arg_count());

        let contents = self.as_str();

        let (index, bytes) = memory.allocate(contents.len(), "String::LowerVal")?;
        bytes.copy_from_slice(contents.as_bytes());

        output[0] = Val::from(index as i32);
        output[1] = Val::from(contents.len() as i32);

        Ok(())
    }

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(range.len(), Self::Target::byte_size());

        let contents = self.as_str();

        let (index, bytes) = memory.allocate(contents.len(), "String::LowerVal")?;
        bytes.copy_from_slice(contents.as_bytes());

        let slice = memory.slice(range)?;
        slice[0..4].copy_from_slice(&(index as u32).to_le_bytes());
        slice[4..8].copy_from_slice(&(contents.len() as u32).to_le_bytes());

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
