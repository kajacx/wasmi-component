use std::ops::Range;
use std::rc::Rc;

use wasmi_component_parser::ValueType;

use crate::helpers::round_up;
use crate::lib_structs::{MemoryAccess, Slice, WasmValue};
use crate::pointers::FatPtr;
use crate::{ConvertResult, DynValue, Lower};

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
}
