use heck::ToSnakeCase;

use crate::parse::Param;

pub struct Func {
    pub module_name: Option<String>,
    pub func_name: String,

    pub params: Vec<Param>,
    pub result: String,
}

impl Func {
    pub fn new(
        module_name: Option<String>,
        func_name: String,
        params: Vec<Param>,
        result: Option<String>,
    ) -> Self {
        Self {
            module_name,
            func_name,
            params,
            result: result.unwrap_or_else(|| "()".to_string()),
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

    pub fn host_params_full(&self) -> String {
        self.params
            .iter()
            .map(|param| format!("{}: {}, ", param.name, param_type(&param.ty)))
            .collect()
    }

    pub fn param_types(&self) -> String {
        self.params
            .iter()
            .map(|param| format!("{}, ", param.ty))
            .collect()
    }

    pub fn param_args(&self) -> String {
        (0..self.params.len())
            .map(|index| format!("args.{index}, "))
            .collect()
    }

    pub fn host_return_type(&self) -> String {
        if PRIMITIVES.contains(&self.result.as_str()) {
            self.result.clone()
        } else {
            format!("impl LowerVal<{}> + 'static", self.result)
        }
    }
}

static PRIMITIVES: &[&str] = &["()", "i32", "u32", "f32"];

fn param_type(ty: &str) -> String {
    if PRIMITIVES.contains(&ty) {
        ty.to_string()
    } else if ty == "String" {
        "&str".to_string()
    } else {
        format!("<{ty} as CompValue>::Borrowed<'_>")
    }
}
