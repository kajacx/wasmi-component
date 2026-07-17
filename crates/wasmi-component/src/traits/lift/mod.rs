use crate::ConvertResult;

mod list;
mod option;
mod primitive;
mod string;
mod tuple;

pub use list::ListAccessor;

pub trait Lift<T> {
    fn lift_owned(&self) -> ConvertResult<T>;

    fn lift_to(&self, target: &mut T) -> ConvertResult<()>;
}
