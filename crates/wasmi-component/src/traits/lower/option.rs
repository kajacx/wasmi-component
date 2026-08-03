use crate::lib_structs::LowerWriter;
use crate::{ComponentValue, ConvertResult, Lower};

impl<T: ComponentValue, L: Lower<T>> Lower<Option<T>> for Option<L> {
    fn lower(&self, writer: &mut impl LowerWriter) -> ConvertResult<()> {
        match self {
            None => writer.write_variant::<Option<T>, _>(0, ()),
            Some(value) => writer.write_variant::<Option<T>, _>(1, value),
        }
    }
}

impl<T: ComponentValue, E: ComponentValue, TL: Lower<T>, EL: Lower<E>> Lower<Result<T, E>>
    for Result<TL, EL>
{
    fn lower(&self, writer: &mut impl LowerWriter) -> ConvertResult<()> {
        match self {
            Ok(value) => writer.write_variant::<Result<T, E>, _>(0, value),
            Err(value) => writer.write_variant::<Result<T, E>, _>(1, value),
        }
    }
}
