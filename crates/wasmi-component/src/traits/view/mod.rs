use anyhow::Result;

mod list;
mod primitive;
mod resource;
mod result;
mod string;
mod tuple;

pub use list::ListAccessor;

pub trait View<T> {
    fn lift_owned(&self) -> Result<T>;

    fn lift_to(&self, target: &mut T) -> Result<()>;
}
