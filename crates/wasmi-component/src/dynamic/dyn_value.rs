use std::rc::Rc;

use wasmi_component_parser::ValueType;

use crate::RecordFields;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DynValue {
    S8(i8),
    S16(i16),
    S32(i32),
    S64(i64),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    F32(f32),
    F64(f64),

    Bool(bool),
    Char(char),
    String(Rc<str>),

    Option(Option<Rc<DynValue>>),
    Result(Result<Rc<DynValue>, Rc<DynValue>>),
    Tuple(Rc<[DynValue]>),
    List(Rc<[DynValue]>),

    Record {
        fields: RecordFields,
    },
    Variant {
        determinant: Rc<str>,
        value: Option<Rc<DynValue>>,
    },
    Enum {
        determinant: Rc<str>,
    },
}

impl DynValue {
    pub fn unit() -> Self {
        Self::new_tuple([])
    }

    pub fn new_s8(value: i8) -> Self {
        Self::S8(value)
    }

    pub fn new_s16(value: i16) -> Self {
        Self::S16(value)
    }

    pub fn new_s32(value: i32) -> Self {
        Self::S32(value)
    }

    pub fn new_s64(value: i64) -> Self {
        Self::S64(value)
    }

    pub fn new_u8(value: u8) -> Self {
        Self::U8(value)
    }

    pub fn new_u16(value: u16) -> Self {
        Self::U16(value)
    }

    pub fn new_u32(value: u32) -> Self {
        Self::U32(value)
    }

    pub fn new_u64(value: u64) -> Self {
        Self::U64(value)
    }

    pub fn new_f32(value: f32) -> Self {
        Self::F32(value)
    }

    pub fn new_f64(value: f64) -> Self {
        Self::F64(value)
    }

    pub fn new_bool(value: bool) -> Self {
        Self::Bool(value)
    }

    pub fn new_char(value: char) -> Self {
        Self::Char(value)
    }

    pub fn new_string(value: impl AsRef<str>) -> Self {
        Self::String(Rc::from(value.as_ref()))
    }

    pub fn new_option(value: Option<DynValue>) -> Self {
        Self::Option(value.map(Rc::new))
    }

    pub fn new_result(value: Result<DynValue, DynValue>) -> Self {
        Self::Result(value.map(Rc::new).map_err(Rc::new))
    }

    pub fn new_tuple(values: impl IntoIterator<Item = DynValue>) -> Self {
        Self::Tuple(values.into_iter().collect())
    }

    pub fn new_list(values: impl IntoIterator<Item = DynValue>) -> Self {
        Self::List(values.into_iter().collect())
    }

    pub fn new_record(values: impl IntoIterator<Item = (Rc<str>, DynValue)>) -> Self {
        Self::Record {
            fields: RecordFields::new(values.into_iter().collect()),
        }
    }

    pub fn new_variant(determinant: impl AsRef<str>, value: Option<DynValue>) -> Self {
        Self::Variant {
            determinant: Rc::from(determinant.as_ref()),
            value: value.map(Rc::new),
        }
    }

    pub fn new_enum(determinant: impl AsRef<str>) -> Self {
        Self::Enum {
            determinant: Rc::from(determinant.as_ref()),
        }
    }

    pub fn is(&self, ty: &ValueType) -> bool {
        match (self, ty) {
            (Self::S8(_), ValueType::S8) => true,
            (Self::S16(_), ValueType::S16) => true,
            (Self::S32(_), ValueType::S32) => true,
            (Self::S64(_), ValueType::S64) => true,

            (Self::U8(_), ValueType::U8) => true,
            (Self::U16(_), ValueType::U16) => true,
            (Self::U32(_), ValueType::U32) => true,
            (Self::U64(_), ValueType::U64) => true,

            (Self::F32(_), ValueType::F32) => true,
            (Self::F64(_), ValueType::F64) => true,

            (Self::Bool(_), ValueType::Bool) => true,
            (Self::Char(_), ValueType::Char) => true,
            (Self::String(_), ValueType::String) => true,

            (Self::Option(None), ValueType::Option(_)) => true,
            (Self::Option(Some(value)), ValueType::Option(ty)) => value.is(ty),
            (Self::Result(Ok(value)), ValueType::Result(ty, _)) => value.is(ty),
            (Self::Result(Err(value)), ValueType::Result(_, ty)) => value.is(ty),
            (Self::Tuple(values), ValueType::Tuple(types)) => {
                values.len() == types.len()
                    && values
                        .iter()
                        .zip(types.iter())
                        .all(|(value, ty)| value.is(ty))
            }
            (Self::List(values), ValueType::List(ty)) => values.iter().all(|value| value.is(ty)),

            (Self::Record { fields }, ValueType::Record { fields: types, .. }) => types
                .iter()
                .all(|(name, ty)| fields.get_field(name).is_some_and(|value| value.is(ty))),

            (Self::Variant { determinant, value }, ValueType::Variant { cases, .. }) => cases
                .iter()
                .find(|(name, _)| name == determinant)
                .is_some_and(|(_, ty)| match (value, ty) {
                    (None, None) => true,
                    (Some(value), Some(ty)) => value.is(ty),
                    _ => false,
                }),

            (Self::Enum { determinant }, ValueType::Enum { cases, .. }) => {
                cases.contains(determinant)
            }

            _ => false,
        }
    }

    pub fn as_s8(&self) -> Option<i8> {
        match self {
            Self::S8(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_s16(&self) -> Option<i16> {
        match self {
            Self::S16(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_s32(&self) -> Option<i32> {
        match self {
            Self::S32(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_s64(&self) -> Option<i64> {
        match self {
            Self::S64(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_u8(&self) -> Option<u8> {
        match self {
            Self::U8(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_u16(&self) -> Option<u16> {
        match self {
            Self::U16(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Self::U32(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::U64(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Self::F32(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::F64(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_char(&self) -> Option<char> {
        match self {
            Self::Char(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value.as_ref()),
            _ => None,
        }
    }

    pub fn as_option(&self) -> Option<Option<&DynValue>> {
        match self {
            Self::Option(value) => Some(value.as_deref()),
            _ => None,
        }
    }

    pub fn as_result(&self) -> Option<Result<&DynValue, &DynValue>> {
        match self {
            Self::Result(value) => Some(value.as_deref().map_err(Rc::as_ref)),
            _ => None,
        }
    }

    pub fn as_tuple(&self) -> Option<&[DynValue]> {
        match self {
            Self::Tuple(value) => Some(value.as_ref()),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&[DynValue]> {
        match self {
            Self::List(value) => Some(value.as_ref()),
            _ => None,
        }
    }

    pub fn as_record(&self) -> Option<&RecordFields> {
        match self {
            Self::Record { fields } => Some(fields),
            _ => None,
        }
    }

    pub fn as_variant(&self) -> Option<(&str, Option<&DynValue>)> {
        match self {
            Self::Variant { determinant, value } => Some((
                determinant.as_ref(),
                value.as_ref().map(|value| value.as_ref()),
            )),
            _ => None,
        }
    }
}

impl Default for DynValue {
    fn default() -> Self {
        Self::unit()
    }
}
