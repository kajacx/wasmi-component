use crate::{ConvertError, ConvertResult};

// TODO: get rid of this? how to fill empty variant fields then?
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum WasmValue {
    Unused,
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

macro_rules! impl_from {
    ($target_ty: ty, $variant: ident) => {
        impl From<$target_ty> for WasmValue {
            fn from(value: $target_ty) -> Self {
                Self::$variant(value as _)
            }
        }
    };
}

impl_from!(i8, I32);
impl_from!(i16, I32);
impl_from!(i32, I32);
impl_from!(i64, I64);

impl_from!(u8, I32);
impl_from!(u16, I32);
impl_from!(u32, I32);
impl_from!(u64, I64);

impl_from!(f32, F32);
impl_from!(f64, F64);

impl WasmValue {
    pub fn i32(self) -> ConvertResult<i32> {
        match self {
            Self::I32(value) => Ok(value),
            Self::I64(value) => Ok(value as i32),
            other => Err(crate::ConvertError::new(format!(
                "cannot convert {other:?} to i32"
            ))),
        }
    }

    pub fn i64(self) -> ConvertResult<i64> {
        match self {
            Self::I64(value) => Ok(value),
            other => Err(crate::ConvertError::new(format!(
                "cannot convert {other:?} to i64"
            ))),
        }
    }

    pub fn f32(self) -> ConvertResult<f32> {
        match self {
            Self::I32(value) => Ok(f32::from_bits(value as u32)),
            Self::I64(value) => Ok(f32::from_bits(value as u32)),
            Self::F32(value) => Ok(value),
            other => Err(crate::ConvertError::new(format!(
                "cannot convert {other:?} to f32"
            ))),
        }
    }

    pub fn f64(self) -> ConvertResult<f64> {
        match self {
            Self::I64(value) => Ok(f64::from_bits(value as u64)),
            Self::F64(value) => Ok(value),
            other => Err(ConvertError::new(format!(
                "cannot convert {other:?} to f64"
            ))),
        }
    }

    pub fn to(self, target: wasmi::ValType) -> ConvertResult<wasmi::Val> {
        match (self, target) {
            (Self::I32(value), wasmi::ValType::I32) => Ok(value.into()),
            (Self::I32(value), wasmi::ValType::I64) => Ok((value as i64).into()),

            (Self::I64(value), wasmi::ValType::I64) => Ok(value.into()),

            (Self::F32(value), wasmi::ValType::I32) => Ok((value.to_bits() as i32).into()),
            (Self::F32(value), wasmi::ValType::I64) => Ok((value.to_bits() as i64).into()),
            (Self::F32(value), wasmi::ValType::F32) => Ok(value.into()),

            (Self::F64(value), wasmi::ValType::I64) => Ok((value.to_bits() as i64).into()),
            (Self::F64(value), wasmi::ValType::F64) => Ok(value.into()),

            (Self::Unused, wasmi::ValType::I32) => Ok(0i32.into()),
            (Self::Unused, wasmi::ValType::I64) => Ok(0i64.into()),
            (Self::Unused, wasmi::ValType::F32) => Ok(0.0f32.into()),
            (Self::Unused, wasmi::ValType::F64) => Ok(0.0f64.into()),

            (other_self, other_target) => Err(ConvertError::new(format!(
                "Cannot convert {other_self:?} to {other_target:?}"
            ))),
        }
    }

    pub fn convert_from_wasmi(values: &[wasmi::Val], target: &mut [Self]) {
        for index in 0..target.len() {
            target[index] = values[index].clone().into()
        }
    }

    pub fn convert_to_wasmi(
        params: &[Self],
        types: &[wasmi::ValType],
        target: &mut [wasmi::Val],
    ) -> ConvertResult<()> {
        for index in 0..target.len() {
            target[index] = params[index].to(types[index])?;
        }
        Ok(())
    }

    pub fn merge_wasmi_vals(val1: wasmi::ValType, val2: wasmi::ValType) -> wasmi::ValType {
        use wasmi::ValType::*;

        match (val1, val2) {
            (I32, I32) => I32,
            (I32, F32) | (F32, I32) => I32,

            (I64, _) | (_, I64) => I64,
            (I32, F64) | (F64, I32) => I64,
            (F32, F64) | (F64, F32) => I64,

            (F32, F32) => F32,
            (F64, F64) => F64,

            (other1, other2) => unimplemented!(
                "Cannot merge wasmi value types {:?} and {:?}",
                other1,
                other2
            ),
        }
    }
}

impl From<wasmi::Val> for WasmValue {
    fn from(value: wasmi::Val) -> Self {
        match value {
            wasmi::Val::I32(val) => Self::I32(val),
            wasmi::Val::I64(val) => Self::I64(val),
            wasmi::Val::F32(val) => Self::F32(val.to_float()),
            wasmi::Val::F64(val) => Self::F64(val.to_float()),
            other => unimplemented!("only basic types are supported, got {other:?} instead"),
        }
    }
}

impl Default for WasmValue {
    fn default() -> Self {
        Self::Unused
    }
}
