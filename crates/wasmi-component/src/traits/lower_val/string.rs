use std::{borrow::Cow, ops::Range};

use anyhow::Result;
use wasmi::Val;

use crate::{CompValue, FatPtr, LowerVal, MemoryAccess};

impl<T: AsStr> LowerVal<String> for T {
    fn lower_args(&self, args: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(args.len(), String::arg_count());

        let contents = self.as_str();
        let ptr = write_contents(contents, memory)?;
        ptr.write_to_args(args);

        Ok(())
    }

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(range.len(), String::byte_size());

        let contents = self.as_str();
        let ptr = write_contents(contents, memory)?;
        ptr.write_to_bytes(memory.slice(range)?);

        Ok(())
    }
}

fn write_contents(contents: &str, memory: &mut impl MemoryAccess) -> Result<FatPtr> {
    let index = memory.allocate(contents.len(), 1)?;
    let slice = memory.slice(index..(index + contents.len()))?;
    slice.copy_from_slice(contents.as_bytes());
    Ok(FatPtr::new(index, contents.len(), 1))
}

// Unfortunately cannot use AsRef<str> directly
pub trait AsStr {
    fn as_str(&self) -> &str;
}

impl<T: AsStr + ?Sized> AsStr for &T {
    fn as_str(&self) -> &str {
        T::as_str(*self)
    }
}

impl<T: AsStr + ?Sized> AsStr for &mut T {
    fn as_str(&self) -> &str {
        T::as_str(*self)
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
