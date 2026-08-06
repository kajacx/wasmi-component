use wasmi_component_parser::ValueType;

use crate::lib_structs::{LiftBytesReader, LiftReader};
use crate::{ComponentValue, ConvertError, ConvertResult, DynValue};

pub(crate) fn dyn_lift<'mem>(
    ty: &ValueType,
    reader: &mut impl LiftReader<'mem>,
) -> ConvertResult<DynValue> {
    match ty {
        ValueType::S8 => Ok(DynValue::new_s8(i8::lift(reader)?)),
        ValueType::S16 => Ok(DynValue::new_s16(i16::lift(reader)?)),
        ValueType::S32 => Ok(DynValue::new_s32(i32::lift(reader)?)),
        ValueType::S64 => Ok(DynValue::new_s64(i64::lift(reader)?)),

        ValueType::U8 => Ok(DynValue::new_u8(u8::lift(reader)?)),
        ValueType::U16 => Ok(DynValue::new_u16(u16::lift(reader)?)),
        ValueType::U32 => Ok(DynValue::new_u32(u32::lift(reader)?)),
        ValueType::U64 => Ok(DynValue::new_u64(u64::lift(reader)?)),

        ValueType::F32 => Ok(DynValue::new_f32(f32::lift(reader)?)),
        ValueType::F64 => Ok(DynValue::new_f64(f64::lift(reader)?)),

        ValueType::Bool => Ok(DynValue::new_bool(bool::lift(reader)?)),
        ValueType::Char => Ok(DynValue::new_char(char::lift(reader)?)),
        ValueType::String => Ok(DynValue::new_string(String::lift(reader)?)),

        ValueType::Option(inner_ty) => {
            reader.read_dyn_variant(ty, |reader, determinant| match determinant {
                0 => Ok(DynValue::new_option(None)),
                1 => Ok(DynValue::new_option(Some(dyn_lift(inner_ty, reader)?))),
                other => Err(ConvertError::new(format!(
                    "invalid determinant {other} in dyn_lift option"
                ))),
            })
        }

        ValueType::Result(ok_ty, err_ty) => {
            reader.read_dyn_variant(ty, |reader, determinant| match determinant {
                0 => Ok(DynValue::new_result(Ok(dyn_lift(ok_ty, reader)?))),
                1 => Ok(DynValue::new_result(Err(dyn_lift(err_ty, reader)?))),
                other => Err(ConvertError::new(format!(
                    "invalid determinant {other} in dyn_lift result"
                ))),
            })
        }

        ValueType::Tuple(fields) => {
            let align = ty.byte_align();
            let mut values = Vec::with_capacity(fields.len());

            for ty in fields.iter() {
                values.push(reader.read_dyn_field(ty, align)?);
            }

            Ok(DynValue::new_tuple(values))
        }

        ValueType::List(inner_ty) => {
            let memory = reader.memory();
            let ptr = reader.read_fat_ptr(inner_ty.byte_size());
            let data = ptr.try_index(memory)?;

            let align = inner_ty.byte_align();
            let mut byte_reader = LiftBytesReader::new(memory, data);
            let mut values = Vec::with_capacity(ptr.count);

            for _ in 0..ptr.count {
                values.push(byte_reader.read_dyn_field(inner_ty, align)?);
            }

            Ok(DynValue::new_list(values))
        }

        ValueType::Record { fields, .. } => {
            let align = ty.byte_align();
            let mut values = Vec::with_capacity(fields.len());

            for (name, ty) in fields.iter() {
                values.push((name.clone(), reader.read_dyn_field(ty, align)?));
            }

            Ok(DynValue::new_record(values))
        }

        ValueType::Variant { name, cases } => reader.read_dyn_variant(ty, |reader, determinant| {
            let (case_name, case_ty) = cases.get(determinant).ok_or_else(|| {
                ConvertError::new(format!("invalid determinant {determinant} in {name}"))
            })?;

            let value = match case_ty {
                Some(case_ty) => Some(dyn_lift(case_ty, reader)?),
                None => None,
            };

            Ok(DynValue::new_variant(case_name, value))
        }),

        ValueType::Enum { name, cases } => {
            let determinant = reader.read_enum_determinant(cases.len(), 1);

            let case_name = cases.get(determinant).ok_or_else(|| {
                ConvertError::new(format!("invalid determinant {determinant} in {name}"))
            })?;

            Ok(DynValue::new_enum(case_name))
        }
    }
}
