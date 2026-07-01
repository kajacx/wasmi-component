use anyhow::Result;
use wasmi::{AsContextMut, Memory};

#[derive(Debug, Clone)]
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
    fn allocate(&mut self, len: usize) -> Result<(usize, &mut [u8])>;
}

impl<T: MemoryAccess> MemoryAccess for &mut T {
    fn allocate(&mut self, len: usize) -> Result<(usize, &mut [u8])> {
        T::allocate(*self, len)
    }
}

impl<'a, C: AsContextMut> MemoryAccess for MemoryAccessFilled<'a, C> {
    fn allocate(&mut self, len: usize) -> Result<(usize, &mut [u8])> {
        let address = self
            .cabi_realloc
            .call(&mut self.ctx, (0, 0, 1, len as i32))? as usize;

        let bytes = self.memory.data_mut(&mut self.ctx);

        Ok((address, &mut bytes[address..(address + len)]))
    }
}
