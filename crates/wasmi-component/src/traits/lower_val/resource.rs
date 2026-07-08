use std::ops::Range;

use anyhow::Result;
use wasmi::Val;

use crate::{Borrow, ComponentValue, LowerVal, MemoryAccess, Own, Resource};

impl<T: Resource> LowerVal<Borrow<T>> for Borrow<T> {
    fn lower_args(&self, args: &mut [Val], _memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(args.len(), Borrow::<T>::arg_count());

        todo!()
    }

    fn lower_bytes(&self, range: Range<usize>, _memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(range.len(), Borrow::<T>::byte_size());

        todo!()
    }
}

impl<T: Resource> LowerVal<Own<T>> for Own<T> {
    fn lower_args(&self, args: &mut [Val], memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(args.len(), Own::<T>::arg_count());

        (self.index as i32).lower_args(args, memory)
    }

    fn lower_bytes(&self, range: Range<usize>, memory: &mut impl MemoryAccess) -> Result<()> {
        debug_assert_eq!(range.len(), Own::<T>::byte_size());

        (self.index as i32).lower_bytes(range, memory)
    }
}
