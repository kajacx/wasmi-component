use wasmi_component_parser::ValueType;

use crate::lib_structs::WasmValue;
use crate::{ConvertResult, DynValue};

pub(crate) fn lift_args_dyn(
    _ty: &ValueType,
    _args: &[WasmValue],
    _memory: &[u8],
) -> ConvertResult<DynValue> {
    todo!("lift_args_dyn")
    // match ty {
    //     ValueType::S8 => Ok(DynValue::new_s8(i8::lift_args(args, memory)?)),
    //     ValueType::S16 => Ok(DynValue::new_s16(i16::lift_args(args, memory)?)),
    //     ValueType::S32 => Ok(DynValue::new_s32(i32::lift_args(args, memory)?)),
    //     ValueType::S64 => Ok(DynValue::new_s64(i64::lift_args(args, memory)?)),

    //     ValueType::U8 => Ok(DynValue::new_u8(u8::lift_args(args, memory)?)),
    //     ValueType::U16 => Ok(DynValue::new_u16(u16::lift_args(args, memory)?)),
    //     ValueType::U32 => Ok(DynValue::new_u32(u32::lift_args(args, memory)?)),
    //     ValueType::U64 => Ok(DynValue::new_u64(u64::lift_args(args, memory)?)),

    //     ValueType::F32 => Ok(DynValue::new_f32(f32::lift_args(args, memory)?)),
    //     ValueType::F64 => Ok(DynValue::new_f64(f64::lift_args(args, memory)?)),

    //     ValueType::Bool => Ok(DynValue::new_bool(bool::lift_args(args, memory)?)),
    //     ValueType::Char => Ok(DynValue::new_char(char::lift_args(args, memory)?)),
    //     ValueType::String => Ok(DynValue::new_string(String::lift_args(args, memory)?)),

    //     ValueType::Option(inner_ty) => {
    //         let value = match args[0].i32()? {
    //             0 => None,
    //             1 => Some(lift_args_dyn(inner_ty, &args[1..], memory)?),
    //             other => {
    //                 return Err(ConvertError::new(format!(
    //                     "invalid determinant {other} in lift_args_dyn option"
    //                 )));
    //             }
    //         };
    //         Ok(DynValue::new_option(value))
    //     }

    //     ValueType::Result(ok_ty, err_ty) => {
    //         let value = match args[0].i32()? {
    //             0 => Ok(lift_args_dyn(
    //                 ok_ty,
    //                 &args[1..(1 + ok_ty.arg_count())],
    //                 memory,
    //             )?),
    //             1 => Err(lift_args_dyn(
    //                 err_ty,
    //                 &args[1..(1 + err_ty.arg_count())],
    //                 memory,
    //             )?),
    //             other => {
    //                 return Err(ConvertError::new(format!(
    //                     "invalid determinant {other} in lift_args_dyn result"
    //                 )));
    //             }
    //         };
    //         Ok(DynValue::new_result(value))
    //     }

    //     ValueType::Tuple(fields) => {
    //         let mut offset = 0;

    //         let values: Result<Rc<[DynValue]>, ConvertError> = fields
    //             .iter()
    //             .map(|field_ty| {
    //                 let field_args = &args[offset..(offset + field_ty.arg_count())];
    //                 offset += field_ty.arg_count();
    //                 lift_args_dyn(field_ty, field_args, memory)
    //             })
    //             .collect();

    //         Ok(DynValue::new_tuple(values?))
    //     }

    //     ValueType::List(inner_ty) => {
    //         let ptr = FatPtr::from_args(args, inner_ty.byte_size())?;
    //         let data = ptr.try_index(memory)?;

    //         let values: Result<Rc<[DynValue]>, ConvertError> = (0..(ptr.count))
    //             .map(|index| {
    //                 let start = index * inner_ty.byte_size();
    //                 let bytes = &data[start..(start + inner_ty.byte_size())];
    //                 lift_bytes_dyn(inner_ty, bytes, memory)
    //             })
    //             .collect();

    //         DynValue::new_list_rc(ty.clone(), values?)
    //     }

    //     ValueType::Record { name, fields } => {
    //         let mut offset = 0;

    //         let values: Result<Vec<(Rc<str>, DynValue)>, ConvertError> = fields
    //             .iter()
    //             .map(|(name, field_ty)| {
    //                 let field_args = &args[offset..(offset + field_ty.arg_count())];
    //                 offset += field_ty.arg_count();
    //                 Ok((name.clone(), lift_args_dyn(field_ty, field_args, memory)?))
    //             })
    //             .collect();

    //         Ok(DynValue::new_record_rc(name.clone(), values?))
    //     }

    //     ValueType::Variant { name, cases } => {
    //         let determinant = args[0].i32()? as usize;

    //         let (_case_name, case_ty) = cases.get(determinant).ok_or_else(|| {
    //             ConvertError::new("invalid determinant in custom variant").with_additional(format!(
    //                 "invalid determinant {determinant} for {name} in lift_args_dyn"
    //             ))
    //         })?;

    //         let value = match case_ty {
    //             Some(inner_ty) => Some(Rc::new(lift_args_dyn(
    //                 inner_ty,
    //                 &args[1..(1 + inner_ty.arg_count())],
    //                 memory,
    //             )?)),
    //             None => None,
    //         };

    //         Ok(DynValue {
    //             ty: ty.clone(),
    //             value: DynInner::Variant { determinant, value },
    //         })
    //     }
    // }
}

pub(crate) fn lift_bytes_dyn(
    _ty: &ValueType,
    _bytes: &[u8],
    _memory: &[u8],
) -> ConvertResult<DynValue> {
    todo!("lift_bytes_dyn")
    // match ty {
    //     ValueType::S8 => Ok(DynValue::new_s8(i8::lift_bytes(bytes, memory)?)),
    //     ValueType::S16 => Ok(DynValue::new_s16(i16::lift_bytes(bytes, memory)?)),
    //     ValueType::S32 => Ok(DynValue::new_s32(i32::lift_bytes(bytes, memory)?)),
    //     ValueType::S64 => Ok(DynValue::new_s64(i64::lift_bytes(bytes, memory)?)),

    //     ValueType::U8 => Ok(DynValue::new_u8(u8::lift_bytes(bytes, memory)?)),
    //     ValueType::U16 => Ok(DynValue::new_u16(u16::lift_bytes(bytes, memory)?)),
    //     ValueType::U32 => Ok(DynValue::new_u32(u32::lift_bytes(bytes, memory)?)),
    //     ValueType::U64 => Ok(DynValue::new_u64(u64::lift_bytes(bytes, memory)?)),

    //     ValueType::F32 => Ok(DynValue::new_f32(f32::lift_bytes(bytes, memory)?)),
    //     ValueType::F64 => Ok(DynValue::new_f64(f64::lift_bytes(bytes, memory)?)),

    //     ValueType::Bool => Ok(DynValue::new_bool(bool::lift_bytes(bytes, memory)?)),
    //     ValueType::Char => Ok(DynValue::new_char(char::lift_bytes(bytes, memory)?)),
    //     ValueType::String => Ok(DynValue::new_string(String::lift_bytes(bytes, memory)?)),

    //     ValueType::Option(inner_ty) => {
    //         let value = match bytes[0] {
    //             0 => None,
    //             1 => Some(lift_bytes_dyn(
    //                 &inner_ty,
    //                 &bytes[ty.byte_align()..],
    //                 memory,
    //             )?),
    //             other => {
    //                 return Err(ConvertError::new(format!(
    //                     "invalid determinant {other} in lift_bytes_dyn option"
    //                 )));
    //             }
    //         };

    //         DynValue::new_option(ty.clone(), value)
    //     }

    //     ValueType::Result(ok_ty, err_ty) => {
    //         let value = match bytes[0] {
    //             0 => Ok(lift_bytes_dyn(&ok_ty, &bytes[ty.byte_align()..], memory)?),
    //             1 => Err(lift_bytes_dyn(&err_ty, &bytes[ty.byte_align()..], memory)?),
    //             other => {
    //                 return Err(ConvertError::new(format!(
    //                     "invalid determinant {other} in lift_bytes_dyn result"
    //                 )));
    //             }
    //         };

    //         DynValue::new_result(ty.clone(), value)
    //     }

    //     ValueType::Tuple(fields) => {
    //         let align = ty.byte_align();
    //         let mut offset = 0;

    //         let mut values = Vec::with_capacity(fields.len());
    //         for field_ty in fields.iter() {
    //             let bytes = &bytes[offset..(offset + field_ty.byte_size())];
    //             values.push(lift_bytes_dyn(field_ty, bytes, memory)?);
    //             offset += round_up(field_ty.byte_size(), align);
    //         }

    //         Ok(DynValue {
    //             ty: ty.clone(),
    //             value: DynInner::new_tuple(values),
    //         })
    //     }

    //     ValueType::List(inner_ty) => {
    //         let ptr = FatPtr::from_bytes(bytes, inner_ty.byte_size())?;
    //         let data = ptr.try_index(memory)?;
    //         let len = ptr.count;
    //         let mut values = Vec::with_capacity(len);
    //         for index in 0..len {
    //             let start = index * inner_ty.byte_size();
    //             let item_bytes = &data[start..(start + inner_ty.byte_size())];
    //             values.push(lift_bytes_dyn(inner_ty, item_bytes, memory)?);
    //         }
    //         Ok(DynValue {
    //             ty: ty.clone(),
    //             value: DynInner::new_list(values),
    //         })
    //     }

    //     ValueType::Record { fields, .. } => {
    //         let mut values = Vec::with_capacity(fields.len());
    //         let mut offset = 0;
    //         let align = ty.byte_align();
    //         for (_name, field_ty) in fields.iter() {
    //             let field_bytes = &bytes[offset..(offset + field_ty.byte_size())];
    //             values.push(lift_bytes_dyn(field_ty, field_bytes, memory)?);
    //             offset += std::cmp::max(field_ty.byte_size(), align);
    //         }
    //         Ok(DynValue {
    //             ty: ty.clone(),
    //             value: DynInner::Record {
    //                 fields: values.into(),
    //             },
    //         })
    //     }

    //     ValueType::Variant { cases, .. } => {
    //         let determinant = bytes.first().copied().unwrap_or_default() as usize;
    //         let case = cases
    //             .get(determinant)
    //             .and_then(|(_name, inner_ty)| inner_ty.as_ref());
    //         let value = match case {
    //             Some(inner_ty) => Some(Rc::new(lift_bytes_dyn(
    //                 inner_ty,
    //                 &bytes[ty.byte_align()..],
    //                 memory,
    //             )?)),
    //             None => None,
    //         };
    //         Ok(DynValue {
    //             ty: ty.clone(),
    //             value: DynInner::Variant { determinant, value },
    //         })
    //     }
    // }
}
