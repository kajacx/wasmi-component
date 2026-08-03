use crate::lib_structs::{MemoryAccess, WasmValue};
use crate::{ConvertError, ConvertResult, DynValue, ValueType};

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
        todo!()
    }
}
