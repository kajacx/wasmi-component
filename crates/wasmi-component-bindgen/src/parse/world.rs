use heck::ToUpperCamelCase;

use crate::parse::Func;

pub struct ParsedWorld {
    pub world_name: String,
    pub imports: Vec<Func>,
    pub exports: Vec<Func>,

    pub imports_name: String,
    pub exports_name: String,

    pub imports_bound: String,
}

impl ParsedWorld {
    pub fn new(world_name: String, imports: Vec<Func>, exports: Vec<Func>) -> Self {
        let imports_name = format!("{}Imports", world_name.to_upper_camel_case());
        let exports_name = format!("{}Exports", world_name.to_upper_camel_case());

        let imports_bound = if !imports.is_empty() {
            format!(": {imports_name}")
        } else {
            "".to_string()
        };

        Self {
            world_name,
            imports,
            exports,

            imports_name,
            exports_name,
            imports_bound,
        }
    }
}
