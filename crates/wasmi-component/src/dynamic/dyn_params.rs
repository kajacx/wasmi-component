use wasmi::ValType;
use wasmi_component_parser::ValueType;

use crate::lib_structs::{MemoryAccess, WasmValue};
use crate::{ComponentValue, ConvertError, ConvertResult, DynValue};

pub(crate) struct DynValueParams<'a> {
    params: &'a [DynValue],
}

impl<'a> DynValueParams<'a> {
    pub fn new(params: &'a [DynValue]) -> Self {
        Self { params }
    }

    pub fn check_params_signature(
        &self,
        other: &[ValueType],
        func_name: impl std::fmt::Display,
    ) -> ConvertResult<()> {
        let ok = self.params.len() == other.len()
            && self
                .params
                .iter()
                .zip(other.iter())
                .all(|(param, ty)| param.is(ty));

        if ok {
            Ok(())
        } else {
            Err(
                ConvertError::new("untyped function was called with wrong params").with_additional(
                    format!(
                        "name: {}, host provided {:?}, but component expected {:?} instead",
                        func_name, self.params, other
                    ),
                ),
            )
        }
    }

    pub fn lower_args(
        &self,
        _args: &mut [WasmValue],
        _memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        //let mut index = 0;
        //for value in self.params {
        // value.lower_args(&mut args[index..(index + value.ty.arg_count())], memory)?; TODO:
        // index += value.ty.arg_count();
        //}
        Ok(())
    }
}

#[allow(unused)]
pub fn dyn_type_to_wasm_params(ty: &ValueType) -> Vec<ValType> {
    match ty {
        ValueType::S8 => i8::arg_types(),
        ValueType::S16 => i16::arg_types(),
        ValueType::S32 => i32::arg_types(),
        ValueType::S64 => i64::arg_types(),

        ValueType::U8 => u8::arg_types(),
        ValueType::U16 => u16::arg_types(),
        ValueType::U32 => u32::arg_types(),
        ValueType::U64 => u64::arg_types(),

        ValueType::F32 => f32::arg_types(),
        ValueType::F64 => f64::arg_types(),

        ValueType::Bool => bool::arg_types(),
        ValueType::Char => char::arg_types(),
        ValueType::String => String::arg_types(),

        _ => todo!("dyn_type_to_wasm_params"),
    }
}
