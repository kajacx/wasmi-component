use crate::lib_structs::LowerWriter;
use crate::{ComponentValue, ConvertResult, Lower};

impl Lower<Self> for () {
    fn lower(&self, _writer: &mut impl LowerWriter) -> ConvertResult<()> {
        Ok(())
    }
}

impl<T: ComponentValue, L: Lower<T>> Lower<(T,)> for (L,) {
    fn lower(&self, writer: &mut impl LowerWriter) -> ConvertResult<()> {
        L::lower(&self.0, writer)
    }
}

impl<T0: ComponentValue, T1: ComponentValue, L0: Lower<T0>, L1: Lower<T1>> Lower<(T0, T1)>
    for (L0, L1)
{
    fn lower(&self, writer: &mut impl LowerWriter) -> ConvertResult<()> {
        let align = <(T0, T1)>::byte_align();
        writer.write_record_field(&self.0, align)?;
        writer.write_record_field(&self.1, align)?;
        Ok(())
    }
}

impl<
    T0: ComponentValue,
    T1: ComponentValue,
    T2: ComponentValue,
    L0: Lower<T0>,
    L1: Lower<T1>,
    L2: Lower<T2>,
> Lower<(T0, T1, T2)> for (L0, L1, L2)
{
    fn lower(&self, writer: &mut impl LowerWriter) -> ConvertResult<()> {
        let align = <(T0, T1, T2)>::byte_align();
        writer.write_record_field(&self.0, align)?;
        writer.write_record_field(&self.1, align)?;
        writer.write_record_field(&self.2, align)?;
        Ok(())
    }
}
