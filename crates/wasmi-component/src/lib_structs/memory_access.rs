use std::ops::Range;

use wasmi::{AsContextMut, Memory};

use crate::{ConvertError, ConvertResult, pointers};

#[derive(Debug, Clone, Copy)]
pub struct MemoryAccessPre {
    pub instance_id: usize,
    pub memory: Memory,
    pub cabi_realloc: wasmi::TypedFunc<(i32, i32, i32, i32), i32>,
}

impl MemoryAccessPre {
    pub fn new(
        instance_id: usize,
        memory: Memory,
        cabi_realloc: wasmi::TypedFunc<(i32, i32, i32, i32), i32>,
    ) -> Self {
        Self {
            instance_id,
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

    pub fn mem_len(&self) -> usize
    where
        C: AsContextMut,
    {
        self.memory.data_size(self.ctx.as_context())
    }
}

pub trait MemoryAccess {
    fn allocate(&mut self, len: usize, align: usize) -> ConvertResult<usize>;

    fn slice(&mut self, range: Range<usize>) -> ConvertResult<&mut [u8]>;
}

impl<T: MemoryAccess> MemoryAccess for &mut T {
    fn allocate(&mut self, len: usize, align: usize) -> ConvertResult<usize> {
        T::allocate(*self, len, align)
    }

    fn slice(&mut self, range: Range<usize>) -> ConvertResult<&mut [u8]> {
        T::slice(*self, range)
    }
}

impl<'a, C: AsContextMut> MemoryAccess for MemoryAccessFilled<'a, C> {
    fn allocate(&mut self, len: usize, align: usize) -> ConvertResult<usize> {
        let address = self
            .cabi_realloc
            .call(&mut self.ctx, (0, 0, align as i32, len as i32))
            .map_err(|err| {
                ConvertError::new("call to allocate failed")
                    .with_additional(format!("len {len}, align {align}"))
                    .with_cause(Box::new(err))
            })?;

        Ok(address as usize)
    }

    fn slice(&mut self, range: Range<usize>) -> ConvertResult<&mut [u8]> {
        let bytes = self.memory.data_mut(self.ctx.as_context_mut());

        let mem_start = pointers::ptr_start(bytes);
        let mem_len = bytes.len();

        bytes.get_mut(range.clone()).ok_or_else(|| {
            ConvertError::new(format!(
                "requested range (start: {:x}, length: {}) is out of bounds for memory (start: {:x}, length: {})",
                range.start,
                range.len(),
                mem_start,
                mem_len,
            ))
        })
    }
}

pub trait Slice {
    fn slice(&self, range: Range<usize>) -> Range<usize>;
}

impl Slice for Range<usize> {
    fn slice(&self, range: Range<usize>) -> Range<usize> {
        let end = self.start + range.end;

        assert!(
            end <= self.end,
            "Slicing range {:?} with {:?} would be out of bounds",
            self,
            range
        );

        (self.start + range.start)..end
    }
}
