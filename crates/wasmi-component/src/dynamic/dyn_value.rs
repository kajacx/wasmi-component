use std::ops::Range;
use std::rc::Rc;

use wasmi_component_parser::ValueType;

use crate::lib_structs::{MemoryAccess, WasmValue};
use crate::{ConvertResult, RecordFields};

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
}

impl DynValue {
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

    pub fn is(&self, ty: &ValueType) -> bool {
        match ty {
            ValueType::S8 => matches!(self, Self::S8(_)),
            ValueType::S16 => matches!(self, Self::S16(_)),
            ValueType::S32 => matches!(self, Self::S32(_)),
            ValueType::S64 => matches!(self, Self::S64(_)),

            ValueType::U8 => matches!(self, Self::U8(_)),
            ValueType::U16 => matches!(self, Self::U16(_)),
            ValueType::U32 => matches!(self, Self::U32(_)),
            ValueType::U64 => matches!(self, Self::U64(_)),

            ValueType::F32 => matches!(self, Self::F32(_)),
            ValueType::F64 => matches!(self, Self::F64(_)),

            ValueType::Bool => matches!(self, Self::F64(_)),
            ValueType::Char => matches!(self, Self::Char(_)),
            ValueType::String => matches!(self, Self::String(_)),

            _ => todo!("value is type?"),
        }
    }

    // TODO: implement the others
    pub fn as_s8(&self) -> Option<i8> {
        match self {
            Self::S8(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_tuple(&self) -> Option<&Rc<[DynValue]>> {
        match self {
            Self::Tuple(value) => Some(value),
            _ => None,
        }
    }

    #[allow(unused)]
    pub(crate) fn lower_args(
        &self,
        ty: &ValueType,
        args: &mut [WasmValue],
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        todo!()
        // match self {
        //     Self::S8(value) => value.lower_args(args, memory),
        //     Self::S16(value) => value.lower_args(args, memory),
        //     Self::S32(value) => value.lower_args(args, memory),
        //     Self::S64(value) => value.lower_args(args, memory),

        //     Self::U8(value) => value.lower_args(args, memory),
        //     Self::U16(value) => value.lower_args(args, memory),
        //     Self::U32(value) => value.lower_args(args, memory),
        //     Self::U64(value) => value.lower_args(args, memory),

        //     Self::F32(value) => value.lower_args(args, memory),
        //     Self::F64(value) => value.lower_args(args, memory),

        //     Self::Bool(value) => value.lower_args(args, memory),
        //     Self::Char(value) => value.lower_args(args, memory),
        //     Self::String(value) => value.lower_args(args, memory),

        //     Self::Option(value) => {
        //         let inner_ty = ty.as_option().expect("type was checked before");
        //         let written = match value {
        //             None => {
        //                 args[0] = WasmValue::I32(0);
        //                 1
        //             }
        //             Some(value) => {
        //                 args[0] = WasmValue::I32(1);
        //                 value.lower_args(
        //                     inner_ty,
        //                     &mut args[1..(1 + inner_ty.arg_count())],
        //                     memory,
        //                 )?;
        //                 1 + inner_ty.arg_count()
        //             }
        //         };

        //         for arg in &mut args[written..] {
        //             *arg = WasmValue::Unused;
        //         }

        //         Ok(())
        //     }
        //     Self::Result(value) => {
        //         let (ok_ty, err_ty) = ty.as_result().expect("type was checked before");
        //         let written = match value {
        //             Ok(ok) => {
        //                 args[0] = WasmValue::I32(0);
        //                 ok.lower_args(ok_ty, &mut args[1..(1 + ok_ty.arg_count())], memory)?;
        //                 1 + ok_ty.arg_count()
        //             }
        //             Err(err) => {
        //                 args[1] = WasmValue::I32(1);
        //                 err.lower_args(err_ty, &mut args[1..(1 + err_ty.arg_count())], memory)?;
        //                 1 + err_ty.arg_count()
        //             }
        //         };

        //         for arg in &mut args[written..] {
        //             *arg = WasmValue::Unused;
        //         }

        //         Ok(())
        //     }
        //     Self::Tuple(fields) => {
        //         let mut index = 0;
        //         let fields_ty = ty.as_tuple().expect("type was checked before");
        //         for (field_ty, field) in fields_ty.iter().zip(fields.iter()) {
        //             field.lower_args(
        //                 field_ty,
        //                 &mut args[index..(index + field_ty.arg_count())],
        //                 memory,
        //             )?;
        //             index += field_ty.arg_count();
        //         }
        //         Ok(())
        //     }
        //     Self::List(contents) => {
        //         let inner_ty = ty.list_type().expect("list type was checked");

        //         let len = inner_ty.byte_size() * contents.len();
        //         let start = memory.allocate(len, inner_ty.byte_align())?;
        //         let mut index = start;

        //         for item in contents.iter() {
        //             item.lower_bytes(inner_ty, index..(index + inner_ty.byte_size()), memory)?;
        //             index += inner_ty.byte_size();
        //         }

        //         let ptr = FatPtr::new(start, contents.len(), inner_ty.byte_size());
        //         ptr.write_to_args(args);
        //         Ok(())
        //     }

        //     Self::Record { fields } => {
        //         let mut index = 0;
        //         let (_name, fields_ty) = ty.as_record().expect("type was checked before");
        //         for (name, field_ty) in fields_ty.iter() {
        //             let field = fields
        //                 .get_field(name.as_ref())
        //                 .expect("type was checked before");
        //             field.lower_args(
        //                 field_ty,
        //                 &mut args[index..(index + field_ty.arg_count())],
        //                 memory,
        //             )?;
        //             index += field_ty.arg_count();
        //         }
        //         Ok(())
        //     }
        //     Self::Variant { determinant, value } => {
        //         // args[0] = WasmValue::I32(*determinant as i32);
        //         // let written = match value {
        //         //     None => 1,
        //         //     Some(value) => {
        //         //         value.lower_args(&mut args[1..(1 + value.ty().arg_count())], memory)?;
        //         //         1 + value.ty().arg_count()
        //         //     }
        //         // };

        //         // for arg in &mut args[written..] {
        //         //     *arg = WasmValue::Unused;
        //         // }

        //         Ok(())
        //     }
        // }
    }

    pub fn lower_bytes(
        &self,
        _ty: &ValueType,
        _range: Range<usize>,
        _memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        todo!("lower_bytes")
        //     match self {
        //         Self::S8(value) => value.lower_bytes(range, memory),
        //         Self::S16(value) => value.lower_bytes(range, memory),
        //         Self::S32(value) => value.lower_bytes(range, memory),
        //         Self::S64(value) => value.lower_bytes(range, memory),

        //         Self::U8(value) => value.lower_bytes(range, memory),
        //         Self::U16(value) => value.lower_bytes(range, memory),
        //         Self::U32(value) => value.lower_bytes(range, memory),
        //         Self::U64(value) => value.lower_bytes(range, memory),

        //         Self::F32(value) => value.lower_bytes(range, memory),
        //         Self::F64(value) => value.lower_bytes(range, memory),

        //         Self::Bool(value) => value.lower_bytes(range, memory),
        //         Self::Char(value) => value.lower_bytes(range, memory),
        //         Self::String(value) => value.lower_bytes(range, memory),

        //         Self::Option(value) => {
        //             let inner_ty = ty.as_option().expect("option type was checked");
        //             let offset = ty.byte_align();

        //             match value {
        //                 None => {
        //                     memory
        //                         .slice(range.start..(range.start + 1))?
        //                         .copy_from_slice(&[0]);
        //                     Ok(())
        //                 }
        //                 Some(value) => {
        //                     memory
        //                         .slice(range.start..(range.start + 1))?
        //                         .copy_from_slice(&[1]);
        //                     value.lower_bytes(
        //                         range.slice(offset..(offset + inner_ty.byte_size())),
        //                         memory,
        //                     )
        //                 }
        //             }
        //         }
        //         Self::Result(value) => {
        //             let (ok_ty, err_ty) = ty.as_result().expect("result type was checked");
        //             let offset = ty.byte_align();

        //             match value {
        //                 Ok(ok) => {
        //                     memory
        //                         .slice(range.start..(range.start + 1))?
        //                         .copy_from_slice(&[0]);
        //                     ok.lower_bytes(range.slice(offset..(offset + ok_ty.byte_size())), memory)
        //                 }
        //                 Err(err) => {
        //                     memory
        //                         .slice(range.start..(range.start + 1))?
        //                         .copy_from_slice(&[1]);
        //                     err.lower_bytes(range.slice(offset..(offset + err_ty.byte_size())), memory)
        //                 }
        //             }
        //         }
        //         Self::Tuple(fields) => {
        //             let align = ty.byte_align();
        //             let mut index = range.start;

        //             for value in fields.iter() {
        //                 value.lower_bytes(index..(index + value.ty().byte_size()), memory)?;
        //                 index += round_up(value.ty().byte_size(), align);
        //             }

        //             Ok(())
        //         }
        //         Self::List(contents) => {
        //             let inner_ty = ty.list_type().expect("list type was checked");
        //             let len = inner_ty.byte_size() * contents.len();
        //             let start = memory.allocate(len, inner_ty.byte_align())?;
        //             let mut index = start;

        //             for item in contents.iter() {
        //                 item.lower_bytes(index..(index + inner_ty.byte_size()), memory)?;
        //                 index += inner_ty.byte_size();
        //             }

        //             let ptr = FatPtr::new(start, contents.len(), inner_ty.byte_size());
        //             ptr.write_to_bytes(memory.slice(range)?);
        //             Ok(())
        //         }

        //         Self::Record { fields } => {
        //             let align = ty.byte_align();
        //             let mut index = range.start;

        //             for field in fields.iter() {
        //                 field.lower_bytes(index..(index + field.ty().byte_size()), memory)?;
        //                 index += round_up(field.ty().byte_size(), align);
        //             }

        //             Ok(())
        //         }
        //         Self::Variant { determinant, value } => {
        //             let offset = ty.byte_align();
        //             // TODO: variants with more than 256 cases
        //             memory.slice(range.start..(range.start + 1))?[0] = *determinant as u8;

        //             if let Some(value) = value {
        //                 value.lower_bytes(
        //                     range.slice(offset..(offset + value.ty().byte_size())),
        //                     memory,
        //                 )?;
        //             }

        //             Ok(())
        //         }
        //     }
    }
}
