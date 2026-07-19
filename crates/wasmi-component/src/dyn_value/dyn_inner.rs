use std::ops::Range;
use std::rc::Rc;

use wasmi_component_parser::ValueType;

use crate::lib_structs::{MemoryAccess, WasmValue};
use crate::pointers::FatPtr;
use crate::{ConvertResult, DynValue, Lower};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum DynInner {
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
        fields: Rc<[DynValue]>,
    },
    Variant {
        determinant: usize,
        value: Option<Rc<DynValue>>,
    },
}

impl DynInner {
    pub fn new_string(value: impl AsRef<str>) -> Self {
        Self::String(Rc::from(value.as_ref()))
    }

    pub fn new_option(value: Option<DynValue>) -> Self {
        Self::Option(value.map(Rc::new))
    }

    pub fn new_result(value: Result<DynValue, DynValue>) -> Self {
        Self::Result(value.map(Rc::new).map_err(Rc::new))
    }

    pub fn new_tuple(fields: impl IntoIterator<Item = DynValue>) -> Self {
        Self::Tuple(fields.into_iter().collect())
    }

    pub fn new_list(values: impl IntoIterator<Item = DynValue>) -> Self {
        Self::List(values.into_iter().collect())
    }

    pub fn lower_args(
        &self,
        ty: &ValueType,
        args: &mut [WasmValue],
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        match self {
            DynInner::S8(value) => value.lower_args(args, memory),
            DynInner::S16(value) => value.lower_args(args, memory),
            DynInner::S32(value) => value.lower_args(args, memory),
            DynInner::S64(value) => value.lower_args(args, memory),

            DynInner::U8(value) => value.lower_args(args, memory),
            DynInner::U16(value) => value.lower_args(args, memory),
            DynInner::U32(value) => value.lower_args(args, memory),
            DynInner::U64(value) => value.lower_args(args, memory),

            DynInner::F32(value) => value.lower_args(args, memory),
            DynInner::F64(value) => value.lower_args(args, memory),

            DynInner::Bool(value) => value.lower_args(args, memory),
            DynInner::Char(value) => value.lower_args(args, memory),
            DynInner::String(value) => value.lower_args(args, memory),

            DynInner::Option(value) => {
                let written = match value {
                    None => {
                        args[0] = WasmValue::I32(0);
                        1
                    }
                    Some(value) => {
                        args[0] = WasmValue::I32(1);
                        value.lower_args(&mut args[1..(1 + value.ty().arg_count())], memory)?;
                        1 + value.ty().arg_count()
                    }
                };

                for arg in &mut args[written..] {
                    *arg = WasmValue::Unused;
                }

                Ok(())
            }
            DynInner::Result(value) => {
                let written = match value {
                    Ok(ok) => {
                        args[0] = WasmValue::I32(0);
                        ok.lower_args(&mut args[1..(1 + ok.ty().arg_count())], memory)?;
                        1 + ok.ty().arg_count()
                    }
                    Err(err) => {
                        args[1] = WasmValue::I32(1);
                        err.lower_args(&mut args[1..(1 + err.ty().arg_count())], memory)?;
                        1 + err.ty().arg_count()
                    }
                };

                for arg in &mut args[written..] {
                    *arg = WasmValue::Unused;
                }

                Ok(())
            }
            DynInner::Tuple(fields) => {
                let mut index = 0;
                for value in fields.iter() {
                    value.lower_args(&mut args[index..(index + value.ty.arg_count())], memory)?;
                    index += value.ty.arg_count();
                }
                Ok(())
            }
            DynInner::List(contents) => {
                let inner_ty = ty.list_type().expect("list type was checked");

                let len = inner_ty.byte_size() * contents.len();
                let start = memory.allocate(len, inner_ty.byte_align())?;
                let mut index = start;

                for item in contents.iter() {
                    item.lower_bytes(index..(index + inner_ty.byte_size()), memory)?;
                    index += inner_ty.byte_size();
                }

                let ptr = FatPtr::new(start, contents.len(), inner_ty.byte_size());
                ptr.write_to_args(args);
                Ok(())
            }

            DynInner::Record { fields } => {
                let mut index = 0;
                for value in fields.iter() {
                    value.lower_args(&mut args[index..(index + value.ty.arg_count())], memory)?;
                    index += value.ty.arg_count();
                }
                Ok(())
            }
            DynInner::Variant { determinant, value } => {
                args[0] = WasmValue::I32(*determinant as i32);
                let written = match value {
                    None => 1,
                    Some(value) => {
                        value.lower_args(&mut args[1..(1 + value.ty().arg_count())], memory)?;
                        1 + value.ty().arg_count()
                    }
                };

                for arg in &mut args[written..] {
                    *arg = WasmValue::Unused;
                }

                Ok(())
            }
        }
    }

    pub fn lower_bytes(
        &self,
        _ty: &ValueType,
        _range: Range<usize>,
        _memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        todo!()
    }
}
