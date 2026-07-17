use std::ops::Range;

use crate::{ComponentValue, ConvertResult, Lower, MemoryAccess, Slice, WasmValue};

impl<T: ComponentValue, TVal: Lower<T>> Lower<Option<T>> for Option<TVal> {
    fn lower_args(
        &self,
        args: &mut [WasmValue],
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(args.len(), Option::<T>::arg_count());

        let written = match self {
            None => {
                args[0] = WasmValue::I32(0);
                1
            }
            Some(value) => {
                args[0] = WasmValue::I32(1);
                value.lower_args(&mut args[1..(1 + T::arg_count())], memory)?;
                1 + T::arg_count()
            }
        };

        for arg in &mut args[written..] {
            *arg = WasmValue::Unused;
        }

        Ok(())
    }

    fn lower_bytes(
        &self,
        range: Range<usize>,
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(range.len(), Option::<T>::byte_size());

        let offset = Option::<T>::byte_align();

        match self {
            None => {
                memory
                    .slice(range.start..(range.start + 1))?
                    .copy_from_slice(&[0]);

                Ok(())
            }
            Some(value) => {
                memory
                    .slice(range.start..(range.start + 1))?
                    .copy_from_slice(&[1]);

                value.lower_bytes(range.slice(offset..(offset + T::byte_size())), memory)
            }
        }
    }
}

impl<T: ComponentValue, E: ComponentValue, TVal: Lower<T>, EVal: Lower<E>> Lower<Result<T, E>>
    for Result<TVal, EVal>
{
    fn lower_args(
        &self,
        args: &mut [WasmValue],
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(args.len(), Result::<T, E>::arg_count());

        let written = match self {
            Ok(ok) => {
                args[0] = WasmValue::I32(0);
                ok.lower_args(&mut args[1..(1 + T::arg_count())], memory)?;
                1 + T::arg_count()
            }
            Err(err) => {
                args[1] = WasmValue::I32(1);
                err.lower_args(&mut args[1..(1 + E::arg_count())], memory)?;
                1 + T::arg_count()
            }
        };

        for arg in &mut args[written..] {
            *arg = WasmValue::Unused;
        }

        Ok(())
    }

    fn lower_bytes(
        &self,
        range: Range<usize>,
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        debug_assert_eq!(range.len(), Result::<T, E>::byte_size());

        let offset = Result::<T, E>::byte_align();

        match self {
            Ok(ok) => {
                memory
                    .slice(range.start..(range.start + 1))?
                    .copy_from_slice(&[0]);

                ok.lower_bytes(range.slice(offset..(offset + T::byte_size())), memory)
            }
            Err(err) => {
                memory
                    .slice(range.start..(range.start + 1))?
                    .copy_from_slice(&[1]);

                err.lower_bytes(range.slice(offset..(offset + E::byte_size())), memory)
            }
        }
    }
}
