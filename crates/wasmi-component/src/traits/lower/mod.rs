use crate::lib_structs::LowerWriter;
use crate::{ComponentValue, ConvertResult};

mod list;
mod option;
mod primitive;
mod string;
mod tuple;

#[blanket::blanket(derive(Ref, Mut, Box, Rc, Arc, Cow))]
pub trait Lower<T: ComponentValue> {
    fn lower(&self, writer: &mut impl LowerWriter) -> ConvertResult<()>;
}
