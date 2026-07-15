use crate::{
    ComponentValue, ConvertResult, LowerValue, MemoryAccess, WasmValue, helpers::round_up,
};

impl LowerValue<Self> for () {
    fn lower_args(
        &self,
        args: &mut [WasmValue],
        _memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(args.len(), Self::arg_count());

        Ok(())
    }

    fn lower_bytes(
        &self,
        range: std::ops::Range<usize>,
        _memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(range.len(), Self::byte_size());

        Ok(())
    }
}

impl<U: ComponentValue, T: LowerValue<U>> LowerValue<(U,)> for (T,) {
    fn lower_args(
        &self,
        args: &mut [WasmValue],
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(args.len(), U::arg_count());

        T::lower_args(&self.0, args, memory)
    }

    fn lower_bytes(
        &self,
        range: std::ops::Range<usize>,
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(range.len(), U::byte_size());

        T::lower_bytes(&self.0, range, memory)
    }
}

impl<U0: ComponentValue, T0: LowerValue<U0>, U1: ComponentValue, T1: LowerValue<U1>>
    LowerValue<(U0, U1)> for (T0, T1)
{
    fn lower_args(
        &self,
        args: &mut [WasmValue],
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(args.len(), <(U0, U1)>::arg_count());

        let mut index = 0;

        T0::lower_args(&self.0, &mut args[index..(index + U0::arg_count())], memory)?;
        index += U0::arg_count();

        T1::lower_args(&self.1, &mut args[index..(index + U1::arg_count())], memory)?;
        index += U1::arg_count();

        debug_assert_eq!(index, <(U0, U1)>::arg_count());

        Ok(())
    }

    fn lower_bytes(
        &self,
        range: std::ops::Range<usize>,
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(range.len(), <(U0, U1)>::byte_size());

        let align = <(U0, U1)>::byte_align();
        let mut index = range.start;

        T0::lower_bytes(&self.0, index..(index + U0::byte_size()), memory)?;
        index += round_up(U0::byte_size(), align);

        T1::lower_bytes(&self.1, index..(index + U1::byte_size()), memory)?;
        index += round_up(U1::byte_size(), align);

        debug_assert_eq!(index, range.end);

        Ok(())
    }
}

impl<
    U0: ComponentValue,
    T0: LowerValue<U0>,
    U1: ComponentValue,
    T1: LowerValue<U1>,
    U2: ComponentValue,
    T2: LowerValue<U2>,
> LowerValue<(U0, U1, U2)> for (T0, T1, T2)
{
    fn lower_args(
        &self,
        args: &mut [WasmValue],
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(args.len(), <(U0, U1, U2)>::arg_count());

        let mut index = 0;

        T0::lower_args(&self.0, &mut args[index..(index + U0::arg_count())], memory)?;
        index += U0::arg_count();

        T1::lower_args(&self.1, &mut args[index..(index + U1::arg_count())], memory)?;
        index += U1::arg_count();

        T2::lower_args(&self.2, &mut args[index..(index + U2::arg_count())], memory)?;
        index += U2::arg_count();

        debug_assert_eq!(index, <(U0, U1, U2)>::arg_count());

        Ok(())
    }

    fn lower_bytes(
        &self,
        range: std::ops::Range<usize>,
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(range.len(), <(U0, U1, U2)>::byte_size());

        let align = <(U0, U1, U2)>::byte_align();
        let mut index = range.start;

        T0::lower_bytes(&self.0, index..(index + U0::byte_size()), memory)?;
        index += round_up(U0::byte_size(), align);

        T1::lower_bytes(&self.1, index..(index + U1::byte_size()), memory)?;
        index += round_up(U1::byte_size(), align);

        T2::lower_bytes(&self.2, index..(index + U2::byte_size()), memory)?;
        index += round_up(U2::byte_size(), align);

        debug_assert_eq!(index, range.end);

        Ok(())
    }
}
