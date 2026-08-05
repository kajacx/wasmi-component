use crate::lib_structs::LowerWriter;
use crate::{ConvertError, ConvertResult, DynValue, ValueType, dyn_lower};

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
        types: &[ValueType],
        writer: &mut impl LowerWriter,
    ) -> ConvertResult<()> {
        for (ty, value) in types.iter().zip(self.params.iter()) {
            dyn_lower(ty, value, writer)?;
        }
        Ok(())
    }
}
