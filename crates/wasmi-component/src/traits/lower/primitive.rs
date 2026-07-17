use std::ops::Range;

use crate::{ComponentValue, ConvertResult, Lower, MemoryAccess, WasmValue};

macro_rules! impl_lower_val_primitive {
    ($main_ty: ty) => {
        impl Lower<Self> for $main_ty {
            fn lower_args(
                &self,
                args: &mut [WasmValue],
                _memory: &mut impl MemoryAccess,
            ) -> ConvertResult<()> {
                debug_assert_eq!(args.len(), Self::arg_count());

                args[0] = WasmValue::from(*self);

                Ok(())
            }

            fn lower_bytes(
                &self,
                range: Range<usize>,
                memory: &mut impl MemoryAccess,
            ) -> ConvertResult<()> {
                debug_assert_eq!(range.len(), Self::byte_size());

                memory.slice(range)?.copy_from_slice(&self.to_le_bytes());

                Ok(())
            }
        }
    };
}

impl_lower_val_primitive!(i8);
impl_lower_val_primitive!(i16);
impl_lower_val_primitive!(i32);
impl_lower_val_primitive!(i64);

impl_lower_val_primitive!(u8);
impl_lower_val_primitive!(u16);
impl_lower_val_primitive!(u32);
impl_lower_val_primitive!(u64);

impl_lower_val_primitive!(f32);
impl_lower_val_primitive!(f64);

impl Lower<Self> for bool {
    fn lower_args(
        &self,
        args: &mut [WasmValue],
        _memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(args.len(), Self::arg_count());

        args[0] = WasmValue::I32(if *self { 1 } else { 0 });

        Ok(())
    }

    fn lower_bytes(
        &self,
        range: Range<usize>,
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(range.len(), Self::byte_size());

        memory.slice(range)?[0] = if *self { 1 } else { 0 };

        Ok(())
    }
}

impl Lower<Self> for char {
    fn lower_args(
        &self,
        args: &mut [WasmValue],
        _memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(args.len(), Self::arg_count());

        args[0] = WasmValue::from(*self as i32);

        Ok(())
    }

    fn lower_bytes(
        &self,
        range: Range<usize>,
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(range.len(), Self::byte_size());

        memory
            .slice(range)?
            .copy_from_slice(&(*self as i32).to_le_bytes());

        Ok(())
    }
}
