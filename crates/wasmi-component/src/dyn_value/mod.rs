use std::ops::Range;
use std::rc::Rc;

use wasmi_component_parser::ValueType;

use crate::lib_structs::{MemoryAccess, WasmValue};
use crate::{ConvertError, ConvertResult};

mod dyn_inner;
mod dyn_lift;
mod dyn_params;

pub(crate) use dyn_inner::*;
pub(crate) use dyn_lift::*;
pub(crate) use dyn_params::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DynValue {
    ty: ValueType,
    value: DynInner,
}

impl DynValue {
    pub fn new_s8(value: i8) -> Self {
        Self {
            ty: ValueType::S8,
            value: DynInner::S8(value),
        }
    }

    pub fn new_s16(value: i16) -> Self {
        Self {
            ty: ValueType::S16,
            value: DynInner::S16(value),
        }
    }

    pub fn new_s32(value: i32) -> Self {
        Self {
            ty: ValueType::S32,
            value: DynInner::S32(value),
        }
    }

    pub fn new_s64(value: i64) -> Self {
        Self {
            ty: ValueType::S64,
            value: DynInner::S64(value),
        }
    }

    pub fn new_u8(value: u8) -> Self {
        Self {
            ty: ValueType::U8,
            value: DynInner::U8(value),
        }
    }

    pub fn new_u16(value: u16) -> Self {
        Self {
            ty: ValueType::U16,
            value: DynInner::U16(value),
        }
    }

    pub fn new_u32(value: u32) -> Self {
        Self {
            ty: ValueType::U32,
            value: DynInner::U32(value),
        }
    }

    pub fn new_u64(value: u64) -> Self {
        Self {
            ty: ValueType::U64,
            value: DynInner::U64(value),
        }
    }

    pub fn new_f32(value: f32) -> Self {
        Self {
            ty: ValueType::F32,
            value: DynInner::F32(value),
        }
    }

    pub fn new_f64(value: f64) -> Self {
        Self {
            ty: ValueType::F64,
            value: DynInner::F64(value),
        }
    }

    pub fn new_bool(value: bool) -> Self {
        Self {
            ty: ValueType::Bool,
            value: DynInner::Bool(value),
        }
    }

    pub fn new_char(value: char) -> Self {
        Self {
            ty: ValueType::Char,
            value: DynInner::Char(value),
        }
    }

    pub fn new_string(value: impl AsRef<str>) -> Self {
        Self {
            ty: ValueType::String,
            value: DynInner::new_string(value),
        }
    }

    /// The `ty` argument is the type of the ENTIRE option.
    /// Use `ValueType::new_option(inner)` if you have the inner type.
    pub fn new_option(ty: ValueType, value: Option<DynValue>) -> ConvertResult<Self> {
        let inner_ty = ty.as_option().ok_or_else(|| {
            ConvertError::new("declared type was not an option type").with_additional(format!(
                "declared type was {}, value was {:?}, use ValueType::new_option for inner type",
                ty, value
            ))
        })?;

        if let Some(inner) = value.as_ref() {
            if inner_ty != inner.ty() {
                return Err(ConvertError::new("dynamic option type does not match")
                    .with_additional(format!(
                        "declared type is {}, but actual value type is {}",
                        inner_ty,
                        inner.ty()
                    )));
            }
        }
        Ok(Self {
            ty,
            value: DynInner::new_option(value),
        })
    }

    /// The `ty` argument is the type of the ENTIRE result.
    /// Use `ValueType::new_result(ok, err)` if you have the inner types.
    pub fn new_result(ty: ValueType, value: Result<DynValue, DynValue>) -> ConvertResult<Self> {
        let (ok_ty, err_ty) = ty.as_result().ok_or_else(|| {
            ConvertError::new("declared type was not a result type")
                .with_additional(format!("declared type was {}, value was {:?}", ty, value))
        })?;

        let (declared, actual, hint) = match value.as_ref() {
            Ok(ok) => (ok_ty, ok.ty(), "ok"),
            Err(err) => (err_ty, err.ty(), "err"),
        };

        if declared != actual {
            return Err(
                ConvertError::new(format!("dynamic result {hint} type does not match"))
                    .with_additional(format!(
                        "declared type is {}, but actual value type is {}",
                        declared, actual
                    )),
            );
        }

        Ok(Self {
            ty,
            value: DynInner::new_result(value),
        })
    }

    /// Values will be put into `Rc<[DynValue]>` for easier cloning.
    ///
    /// If you already have an `Rc<[DynValue]>`, you can use `new_tuple_rc` instead.
    pub fn new_tuple(values: impl IntoIterator<Item = DynValue>) -> Self {
        Self::new_tuple_rc(values.into_iter().collect())
    }

    pub fn new_tuple_rc(values: Rc<[DynValue]>) -> Self {
        let types = values.iter().map(|value| value.ty().clone()).collect();

        Self {
            ty: ValueType::Tuple(types),
            value: DynInner::Tuple(values),
        }
    }

    /// The `ty` argument is the type of the ENTIRE list.
    /// Use `ValueType::new_list(inner)` if you have the inner type.
    ///
    /// Values will be put into `Rc<[DynValue]>` for easier cloning.
    ///
    /// If you already have an `Rc<[DynValue]>`, you can use `new_tuple_rc` instead.
    pub fn new_list(
        ty: ValueType,
        values: impl IntoIterator<Item = DynValue>,
    ) -> ConvertResult<Self> {
        Self::new_list_rc(ty, values.into_iter().collect())
    }

    /// The `ty` argument is the type of the ENTIRE list.
    /// Use `ValueType::new_list(inner)` if you have the inner type.
    pub fn new_list_rc(ty: ValueType, values: Rc<[DynValue]>) -> ConvertResult<Self> {
        let inner_ty = ty.as_list().ok_or_else(|| {
            ConvertError::new("declared type was not a list type").with_additional(format!(
                "declared type was {}, use ValueType::new_list for inner type",
                ty
            ))
        })?;

        for item in values.iter() {
            if inner_ty != item.ty() {
                return Err(ConvertError::new("dynamic list item type does not match")
                    .with_additional(format!(
                        "declared type is {}, but actual value type is {}",
                        inner_ty,
                        item.ty()
                    )));
            }
        }

        Ok(Self {
            ty,
            value: DynInner::List(values),
        })
    }

    pub fn new_record_rc(
        name: Rc<str>,
        values: impl IntoIterator<Item = (Rc<str>, DynValue)>,
    ) -> Self {
        let mut field_types = Vec::new();
        let mut field_values = Vec::new();

        for (name, value) in values.into_iter() {
            field_types.push((Rc::from(name), value.ty().clone()));
            field_values.push(value);
        }

        Self {
            ty: ValueType::Record {
                name,
                fields: Rc::from(field_types),
            },
            value: DynInner::Record {
                fields: Rc::from(field_values),
            },
        }
    }

    pub fn new_variant(
        ty: ValueType,
        determinant: usize,
        value: Option<DynValue>,
    ) -> ConvertResult<Self> {
        let (name, cases) = ty.as_variant().ok_or_else(|| {
            ConvertError::new("declared type was not a variant type")
                .with_additional(format!("declared type was {} instead", ty))
        })?;

        let (case_name, case_ty) = cases.get(determinant).ok_or_else(|| {
            ConvertError::new("invalid determinant in custom variant").with_additional(format!(
                "variant type is {}, but determinant was {}",
                ty, determinant
            ))
        })?;

        let declared_ty = case_ty.as_ref();
        let actual_ty = value.as_ref().map(DynValue::ty);
        if declared_ty != actual_ty {
            return Err(
                ConvertError::new("variant value does not match the declared case type")
                    .with_additional(format!(
                        "declared type for {}::{} is {:?}, but value type is {:?}",
                        name, case_name, declared_ty, actual_ty
                    )),
            );
        }

        Ok(Self {
            ty,
            value: DynInner::Variant {
                determinant,
                value: value.map(Rc::new),
            },
        })
    }

    pub fn ty(&self) -> &ValueType {
        &self.ty
    }

    pub fn lower_args(
        &self,
        args: &mut [WasmValue],
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        self.value.lower_args(&self.ty, args, memory)
    }

    pub fn lower_bytes(
        &self,
        range: Range<usize>,
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        self.value.lower_bytes(&self.ty, range, memory)
    }
}
