use std::ops::Range;

use anyhow::{Context, Result};
use wasmi::{AsContextMut, Memory, Val};

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
    fn allocate(&mut self, len: usize, align: usize) -> Result<usize>;

    fn slice(&mut self, range: Range<usize>) -> Result<&mut [u8]>;
}

impl<T: MemoryAccess> MemoryAccess for &mut T {
    fn allocate(&mut self, len: usize, align: usize) -> Result<usize> {
        T::allocate(*self, len, align)
    }

    fn slice(&mut self, range: Range<usize>) -> Result<&mut [u8]> {
        T::slice(*self, range)
    }
}

impl<'a, C: AsContextMut> MemoryAccess for MemoryAccessFilled<'a, C> {
    fn allocate(&mut self, len: usize, align: usize) -> Result<usize> {
        let address = self
            .cabi_realloc
            .call(&mut self.ctx, (0, 0, align as i32, len as i32))? as usize;

        Ok(address)
    }

    fn slice(&mut self, range: Range<usize>) -> Result<&mut [u8]> {
        let bytes = self.memory.data_mut(self.ctx.as_context_mut());
        Ok(bytes.get_mut(range).context("MemoryAccess::slice")?)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FatPtr {
    /// Address where the memory starts
    pub start: usize,

    /// The number of items
    pub count: usize,

    /// Size in bytes of one item
    pub size: usize,
}

impl FatPtr {
    pub fn new(start: usize, count: usize, size: usize) -> Self {
        Self { start, count, size }
    }

    pub fn from_args(args: &[Val], size: usize) -> Result<Self> {
        debug_assert_eq!(args.len(), 2);

        let start = args[0].i32().context("FatPtr::from_args start")? as usize;
        let count = args[1].i32().context("FatPtr::from_args count")? as usize;

        Ok(Self { start, count, size })
    }

    pub fn from_bytes(bytes: &[u8], size: usize) -> Result<Self> {
        debug_assert_eq!(bytes.len(), 8);

        let start = u32::from_le_bytes(bytes[0..4].try_into()?) as usize;
        let count = u32::from_le_bytes(bytes[4..8].try_into()?) as usize;

        Ok(Self { start, count, size })
    }

    pub fn as_range(&self) -> Range<usize> {
        self.start..(self.start + (self.count * self.size))
    }

    pub fn try_index<'a>(&self, bytes: &'a [u8]) -> Result<&'a [u8]> {
        bytes.get(self.as_range()).with_context(|| {
            format!(
                "Tried to index memory of size {} at {} with length {}",
                bytes.len(),
                self.start,
                self.count * self.size
            )
        })
    }

    pub fn write_to_args(&self, args: &mut [Val]) {
        debug_assert_eq!(args.len(), 2);

        args[0] = Val::I32(self.start as _);
        args[1] = Val::I32(self.count as _);
    }

    pub fn write_to_bytes(&self, bytes: &mut [u8]) {
        debug_assert_eq!(bytes.len(), 8);

        bytes[0..4].copy_from_slice(&(self.start as u32).to_le_bytes());
        bytes[4..8].copy_from_slice(&(self.count as u32).to_le_bytes());
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
