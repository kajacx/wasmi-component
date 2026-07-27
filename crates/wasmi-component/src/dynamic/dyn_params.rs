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
        if self.params.iter().map(|param| param.ty()).eq(other) {
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

    pub fn arg_count(&self) -> usize {
        self.params.iter().map(|param| param.ty().arg_count()).sum()
    }

    pub fn lower_args(
        &self,
        args: &mut [WasmValue],
        memory: &mut impl MemoryAccess,
    ) -> ConvertResult<()> {
        let mut index = 0;
        for value in self.params {
            value.lower_args(&mut args[index..(index + value.ty.arg_count())], memory)?;
            index += value.ty.arg_count();
        }
        Ok(())
    }
}

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
