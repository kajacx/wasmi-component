use std::ops::Range;

use anyhow::{Context, Result};
use wasmi::{AsContextMut, Memory};

#[derive(Debug, Clone, Copy)]
pub struct MemoryAccessPre {
    pub memory: Memory,
    pub cabi_realloc: wasmi::TypedFunc<(i32, i32, i32, i32), i32>,
}

impl MemoryAccessPre {
    pub fn new(memory: Memory, cabi_realloc: wasmi::TypedFunc<(i32, i32, i32, i32), i32>) -> Self {
        Self {
            memory,
            cabi_realloc,
        }
    }

    pub fn fill<C: AsContextMut>(&self, ctx: C) -> MemoryAccessFilled<'_, C> {
        MemoryAccessFilled::new(self, ctx)
    }
}

#[derive(Debug)]
pub struct MemoryAccessFilled<'a, C> {
    memory: &'a Memory,
    cabi_realloc: &'a wasmi::TypedFunc<(i32, i32, i32, i32), i32>,
    ctx: C,
}

impl<'a, C> MemoryAccessFilled<'a, C> {
    pub fn new(pre: &'a MemoryAccessPre, ctx: C) -> Self {
        Self {
            memory: &pre.memory,
            cabi_realloc: &pre.cabi_realloc,
            ctx,
        }
    }
}

pub trait MemoryAccess {
    fn allocate(&mut self, len: usize, hint: &str) -> Result<(usize, &mut [u8])>;
}

impl<T: MemoryAccess> MemoryAccess for &mut T {
    fn allocate(&mut self, len: usize, hint: &str) -> Result<(usize, &mut [u8])> {
        T::allocate(*self, len, hint)
    }
}

impl<'a, C: AsContextMut> MemoryAccess for MemoryAccessFilled<'a, C> {
    fn allocate(&mut self, len: usize, hint: &str) -> Result<(usize, &mut [u8])> {
        let address = self
            .cabi_realloc
            .call(&mut self.ctx, (0, 0, 1, len as i32))? as usize;

        let bytes = self.memory.data_mut(self.ctx.as_context_mut());
        let slice = FatPtr::new(address, len).try_index_mut(bytes, hint)?;

        Ok((address, slice))
    }
}

#[derive(Debug)]
pub(crate) struct FatPtr {
    pub start: usize,
    pub len: usize,
}

impl FatPtr {
    pub fn new(start: usize, len: usize) -> Self {
        Self { start, len }
    }

    pub fn from_data(data: &[u8], addr: usize) -> Self {
        let start = u32::from_le_bytes(data[addr..(addr + 4)].try_into().unwrap()) as usize;
        let len = u32::from_le_bytes(data[(addr + 4)..(addr + 8)].try_into().unwrap()) as usize;
        Self { start, len }
    }

    pub fn as_range(&self) -> Range<usize> {
        self.start..(self.start + self.len)
    }

    pub fn try_index<'a>(&self, bytes: &'a [u8], hint: &str) -> Result<&'a [u8]> {
        bytes.get(self.as_range()).with_context(|| {
            format!(
                "Tried to index memory or size {} at {} with length {}, hint: {}",
                bytes.len(),
                self.start,
                self.len,
                hint
            )
        })
    }

    pub fn try_index_mut<'a>(&self, bytes: &'a mut [u8], hint: &str) -> Result<&'a mut [u8]> {
        let bytes_len = bytes.len();
        bytes.get_mut(self.as_range()).with_context(|| {
            format!(
                "Tried to index memory or size {} at {} with length {}, hint: {}",
                bytes_len, self.start, self.len, hint
            )
        })
    }
}
