use std::fmt::Debug;
use std::ops::Range;

use crate::lib_structs::{LiftBytesReader, LowerBytesWriter, MemoryAccess};
use crate::{ComponentValue, ConvertResult, DynValue, ValueType, helpers::round_up};
use crate::{ConvertError, Lift, Lower, dyn_lift, dyn_lower};

pub fn assert_equal_repr_typed<T: ComponentValue + Lower<T> + PartialEq + Debug>(
    value: &T,
    memory: &[u8],
) {
    let mut memory_access = TestMemAccess::new(T::byte_size());
    let mut byte_writer = LowerBytesWriter::new(&mut memory_access, 0);

    value.lower(&mut byte_writer).unwrap();
    assert_eq!(
        memory_access.mem, memory,
        "value {value:?} serialized incorrectly to memory"
    );

    let mut byte_reader = LiftBytesReader::new(memory, &memory[0..T::byte_size()]);

    let result = T::lift(&mut byte_reader).unwrap().lift_owned().unwrap();
    assert_eq!(
        &result, value,
        "value read from bytes does not match original value"
    )
}

pub fn assert_equal_repr_dyn(ty: &ValueType, value: &DynValue, memory: &[u8]) {
    assert!(value.is(ty), "value {value:?} is not of type {ty:?}");

    let mut memory_access = TestMemAccess::new(ty.byte_size());
    let mut byte_writer = LowerBytesWriter::new(&mut memory_access, 0);

    dyn_lower(ty, value, &mut byte_writer).unwrap();
    assert_eq!(
        memory_access.mem, memory,
        "value {value:?} serialized incorrectly to memory"
    );

    let mut byte_reader = LiftBytesReader::new(memory, &memory[0..ty.byte_size()]);

    let result = dyn_lift(ty, &mut byte_reader).unwrap();
    assert_eq!(
        &result, value,
        "value read from bytes does not match original value"
    )
}

struct TestMemAccess {
    mem: Vec<u8>,
}

impl TestMemAccess {
    fn new(initial_size: usize) -> Self {
        Self {
            mem: vec![0; initial_size],
        }
    }
}

impl MemoryAccess for TestMemAccess {
    fn allocate(&mut self, len: usize, align: usize) -> ConvertResult<usize> {
        let start = round_up(self.mem.len(), align);
        self.mem.resize(start + len, 0);
        Ok(start)
    }

    fn slice(&mut self, range: Range<usize>) -> ConvertResult<&mut [u8]> {
        let len = self.mem.len();
        self.mem.get_mut(range.clone()).ok_or_else(|| {
            ConvertError::new(format!(
                "range {range:?} out of bounds for memory of length {len:?}",
            ))
        })
    }

    fn re_borrow(&mut self) -> &mut impl MemoryAccess {
        self
    }
}
