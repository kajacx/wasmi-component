use std::ops::Range;

use crate::{ConvertError, ConvertResult, WasmValue};

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct FatPtr {
    /// Address where the memory starts
    pub start: usize,

    /// The number of items
    pub count: usize,

    /// Size in bytes of one item
    pub size: usize,
}

impl AsRef<FatPtr> for &FatPtr {
    fn as_ref(&self) -> &FatPtr {
        self
    }
}

impl FatPtr {
    pub fn new(start: usize, count: usize, size: usize) -> Self {
        Self { start, count, size }
    }

    pub fn from_args(args: &[WasmValue], size: usize) -> ConvertResult<Self> {
        debug_assert_eq!(args.len(), 2);

        let start = args[0].i32()? as usize;
        let count = args[1].i32()? as usize;

        Ok(Self { start, count, size })
    }

    pub fn from_bytes(bytes: &[u8], size: usize) -> ConvertResult<Self> {
        debug_assert_eq!(bytes.len(), 8);

        let start = u32::from_le_bytes(bytes[0..4].try_into().unwrap()) as usize;
        let count = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;

        Ok(Self { start, count, size })
    }

    pub fn as_range(&self) -> Range<usize> {
        self.start..(self.start + (self.count * self.size))
    }

    pub fn try_index<'a>(&self, bytes: &'a [u8]) -> ConvertResult<&'a [u8]> {
        bytes.get(self.as_range()).ok_or_else(|| {
            ConvertError::new(format!(
                "Tried to index memory of size {} at {} with length {}",
                bytes.len(),
                self.start,
                self.count * self.size
            ))
        })
    }

    pub fn write_to_args(&self, args: &mut [WasmValue]) {
        debug_assert_eq!(args.len(), 2);

        args[0] = WasmValue::I32(self.start as _);
        args[1] = WasmValue::I32(self.count as _);
    }

    pub fn write_to_bytes(&self, bytes: &mut [u8]) {
        debug_assert_eq!(bytes.len(), 8);

        bytes[0..4].copy_from_slice(&(self.start as u32).to_le_bytes());
        bytes[4..8].copy_from_slice(&(self.count as u32).to_le_bytes());
    }
}

pub(crate) struct PtrView<'a> {
    start: usize,
    data: &'a [u8],
}

impl<'a> PtrView<'a> {
    pub fn new(ptr: &'a [u8], offset: usize) -> Self {
        Self {
            start: ptr_start(ptr) - offset,
            data: ptr,
        }
    }
}

impl std::fmt::Debug for PtrView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ptr {{ start: 0x{:x}, len: {}",
            self.start,
            self.data.len()
        )?;
        if self.data.len() <= 32 {
            write!(f, ", data: 0x")?;
            for byte in self.data {
                write!(f, "{:02x}", byte)?;
            }
        }
        write!(f, " }}")
    }
}

pub(crate) fn ptr_start<T>(pointer: &[T]) -> usize {
    pointer.as_ptr() as usize
}
