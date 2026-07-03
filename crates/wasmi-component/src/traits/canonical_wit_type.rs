use crate::Lift;

pub trait CanonicalWitType {
    type ReturnType: Lift;

    fn argument_count() -> usize;
}

impl CanonicalWitType for i32 {
    type ReturnType = Self;

    fn argument_count() -> usize {
        1
    }
}

impl CanonicalWitType for u32 {
    type ReturnType = Self;

    fn argument_count() -> usize {
        1
    }
}

impl CanonicalWitType for f32 {
    type ReturnType = Self;

    fn argument_count() -> usize {
        1
    }
}

impl CanonicalWitType for () {
    type ReturnType = Self;

    fn argument_count() -> usize {
        0
    }
}

impl<T0: CanonicalWitType> CanonicalWitType for (T0,) {
    type ReturnType = T0::ReturnType;

    fn argument_count() -> usize {
        T0::argument_count()
    }
}

impl<T0: CanonicalWitType, T1: CanonicalWitType> CanonicalWitType for (T0, T1) {
    type ReturnType = (T0::ReturnType, T1::ReturnType);

    fn argument_count() -> usize {
        T0::argument_count() + T1::argument_count()
    }
}
