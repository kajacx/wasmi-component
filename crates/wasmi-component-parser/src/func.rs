use crate::ValueType;

pub struct Func {
    pub module_name: Option<String>,
    pub func_name: String,

    pub params: Vec<(String, ValueType)>,
    pub result: ValueType,
}

impl Func {
    pub fn new(
        module_name: Option<String>,
        func_name: String,
        params: Vec<(String, ValueType)>,
        result: ValueType,
    ) -> Self {
        Self {
            module_name,
            func_name,
            params,
            result,
        }
    }
}
