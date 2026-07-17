use crate::{ConvertResult, Lift};

macro_rules! impl_view_primitive {
    ($ty: ty) => {
        impl Lift<Self> for $ty {
            fn lift_owned(&self) -> ConvertResult<Self> {
                Ok(*self)
            }

            fn lift_to(&self, target: &mut Self) -> ConvertResult<()> {
                *target = *self;
                Ok(())
            }
        }
    };
}

impl_view_primitive!(i8);
impl_view_primitive!(i16);
impl_view_primitive!(i32);
impl_view_primitive!(i64);

impl_view_primitive!(u8);
impl_view_primitive!(u16);
impl_view_primitive!(u32);
impl_view_primitive!(u64);

impl_view_primitive!(f32);
impl_view_primitive!(f64);

impl_view_primitive!(bool);
impl_view_primitive!(char);
