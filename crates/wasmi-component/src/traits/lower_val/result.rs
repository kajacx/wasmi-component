use std::ops::Range;

use anyhow::Result;
use wasmi::Val;

use crate::{ComponentValue, LowerVal, MemoryAccess, Slice};

impl<T: ComponentValue, E: ComponentValue, TVal: LowerVal<T>, EVal: LowerVal<E>>
    LowerVal<Result<T, E>> for Result<TVal, EVal>
{
    fn lower_args(&self, args: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(args.len(), Result::<T, E>::arg_count());

        match self {
            Ok(ok) => {
                args[0] = Val::I32(0);
                ok.lower_args(&mut args[1..(1 + T::arg_count())], memory)
            }
            Err(err) => {
                args[1] = Val::I32(1);
                err.lower_args(&mut args[1..(1 + E::arg_count())], memory)
            }
        }
    }

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess) -> Result<()> {
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

impl<T: ComponentValue, TVal: LowerVal<T>> LowerVal<Option<T>> for Option<TVal> {
    fn lower_args(&self, args: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(args.len(), Option::<T>::arg_count());

        match self {
            None => {
                args[0] = Val::I32(0);
                Ok(())
            }
            Some(val) => {
                args[0] = Val::I32(1);
                val.lower_args(&mut args[1..(1 + T::arg_count())], memory)
            }
        }
    }

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(range.len(), Option::<T>::byte_size());

        let offset = Option::<T>::byte_align();

        match self {
            None => {
                memory
                    .slice(range.start..(range.start + 1))?
                    .copy_from_slice(&[0]);

                Ok(())
            }
            Some(val) => {
                memory
                    .slice(range.start..(range.start + 1))?
                    .copy_from_slice(&[1]);

                val.lower_bytes(range.slice(offset..(offset + T::byte_size())), memory)
            }
        }
    }
}
