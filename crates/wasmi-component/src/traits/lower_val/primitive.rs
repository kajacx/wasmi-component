use std::ops::Range;

use wasmi::Val;

use crate::{ComponentValue, ConvertResult, LowerVal, MemoryAccess};

macro_rules! impl_lower_val_primitive {
    ($main_ty: ty, $wasmi_ty: ty) => {
        impl LowerVal<Self> for $main_ty {
            fn lower_args(
                &self,
                args: &mut [Val],
                _memory: &mut impl MemoryAccess,
            ) -> ConvertResult<()> {
                debug_assert_eq!(args.len(), Self::arg_count());

                args[0] = Val::from(*self as $wasmi_ty);

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

impl_lower_val_primitive!(i8, i32);
impl_lower_val_primitive!(i16, i32);
impl_lower_val_primitive!(i32, i32);
impl_lower_val_primitive!(i64, i64);

impl_lower_val_primitive!(u8, i32);
impl_lower_val_primitive!(u16, i32);
impl_lower_val_primitive!(u32, i32);
impl_lower_val_primitive!(u64, i64);

impl_lower_val_primitive!(f32, f32);
impl_lower_val_primitive!(f64, f64);

impl LowerVal<Self> for bool {
    fn lower_args(&self, args: &mut [Val], _memory: &mut impl MemoryAccess) -> ConvertResult<()> {
        debug_assert_eq!(args.len(), Self::arg_count());

        args[0] = Val::I32(if *self { 1 } else { 0 });

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

impl LowerVal<Self> for char {
    fn lower_args(&self, args: &mut [Val], _memory: &mut impl MemoryAccess) -> ConvertResult<()> {
        debug_assert_eq!(args.len(), Self::arg_count());

        args[0] = Val::from(*self as i32);

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
