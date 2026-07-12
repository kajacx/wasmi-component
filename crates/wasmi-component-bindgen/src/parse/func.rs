use heck::ToSnakeCase;

use crate::parse::{LowerArg, Param, ParamType};

pub struct Func {
    pub module_name: Option<String>,
    pub func_name: String,

    pub params: Vec<Param>,
    pub result: ParamType,
}

impl Func {
    pub fn new(
        module_name: Option<String>,
        func_name: String,
        params: Vec<Param>,
        result: ParamType,
    ) -> Self {
        Self {
            module_name,
            func_name,
            params,
            result,
        }
    }

    pub fn rust_name(&self) -> String {
        self.func_name.to_snake_case()
    }

    pub fn exported_name(&self) -> String {
        if let Some(module) = self.module_name.as_ref() {
            format!("{module}#{}", self.func_name)
        } else {
            self.func_name.clone()
        }
    }
    pub fn param_arg_indexes(&self) -> String {
        (0..self.params.len())
            .map(|index| format!("params.{index}, "))
            .collect()
    }

    pub fn params_full_lift(&self) -> String {
        self.params
            .iter()
            .map(|param| format!("{}: {}, ", param.name, param.ty.lift))
            .collect()
    }

    pub fn params_full_lower(&self) -> String {
        self.params
            .iter()
            .map(|param| {
                format!(
                    "{}: {}, ",
                    param.name,
                    match &param.ty.lower {
                        LowerArg::LowerValue => format!("impl LowerValue<{}>", param.ty.canon),
                        LowerArg::Specific(specific) => specific.to_string(),
                    }
                )
            })
            .collect()
    }

    pub fn param_types_canon(&self) -> String {
        self.params
            .iter()
            .map(|param| format!("{}, ", param.ty.canon))
            .collect()
    }

    pub fn param_names(&self) -> String {
        self.params
            .iter()
            .map(|param| format!("{}, ", param.name))
            .collect()
    }
}
