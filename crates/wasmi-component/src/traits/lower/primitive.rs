use crate::lib_structs::LowerWriter;
use crate::{ConvertResult, Lower};

macro_rules! impl_lower_val_primitive {
    ($main_ty: ty, $write_fn: ident) => {
        impl Lower<Self> for $main_ty {
            fn lower(&self, writer: &mut impl LowerWriter) -> ConvertResult<()> {
                writer.$write_fn(*self as _);
                Ok(())
            }
        }
    };
}

impl_lower_val_primitive!(i8, write_u8);
impl_lower_val_primitive!(i16, write_u16);
impl_lower_val_primitive!(i32, write_u32);
impl_lower_val_primitive!(i64, write_u64);

impl_lower_val_primitive!(u8, write_u8);
impl_lower_val_primitive!(u16, write_u16);
impl_lower_val_primitive!(u32, write_u32);
impl_lower_val_primitive!(u64, write_u64);

impl_lower_val_primitive!(f32, write_f32);
impl_lower_val_primitive!(f64, write_f64);

impl Lower<Self> for bool {
    fn lower(&self, writer: &mut impl LowerWriter) -> ConvertResult<()> {
        writer.write_u8(if *self { 1 } else { 0 });
        Ok(())
    }
}

impl Lower<Self> for char {
    fn lower(&self, writer: &mut impl LowerWriter) -> ConvertResult<()> {
        writer.write_u32(*self as u32);
        Ok(())
    }
}
