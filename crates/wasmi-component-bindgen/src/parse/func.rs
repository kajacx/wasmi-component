use heck::ToSnakeCase;
use wit_parser::{Function, Interface, UnresolvedPackage, WorldKey};

use crate::parse::Param;

pub struct Func {
    pub rust_name: String,
    pub exported_name: String,
    pub imported_module: String,
    pub imported_name: String,

    pub param_full: String,
    pub param_types: String,
    pub param_names: String,

    pub result_type: String,
}

impl Func {
    pub fn new(
        pkg: &UnresolvedPackage,
        func: &Function,
        key: &WorldKey,
        interface: Option<&Interface>,
        params: Vec<Param>,
        result: Option<String>,
    ) -> Self {
        let prefix_name = interface.map(|iface| {
            iface
                .name
                .clone()
                .unwrap_or_else(|| key.clone().unwrap_name())
        });

        let rust_name = if let Some(prefix) = prefix_name {
            format!("{}_{}", prefix.to_snake_case(), func.name.to_snake_case())
        } else {
            func.name.to_snake_case()
        };

        let interface_name = interface.and_then(|iface| iface.name.as_ref());
        let (imported_module, imported_name, exported_name) = match (interface, interface_name) {
            (Some(_), Some(interface_name)) => {
                let namespace = &pkg.name.namespace;
                let pkg_name = &pkg.name.name;
                let version = pkg
                    .name
                    .version
                    .as_ref()
                    .map_or("".to_string(), |v| format!("@{v}"));

                let func_name = &func.name;
                (
                    "$root".to_string(),
                    func.name.to_string(),
                    format!("{namespace}:{pkg_name}/{interface_name}{version}#{func_name}"),
                )
            }
            (Some(_), None) => (
                "$root".to_string(),
                func.name.to_string(),
                format!("{}#{}", key.clone().unwrap_name(), func.name),
            ),
            (None, _) => (
                "$root".to_string(),
                func.name.to_string(),
                func.name.to_string(),
            ),
        };

        let param_full = params
            .iter()
            .map(|param| format!("{}: {}, ", param.name, param.ty))
            .collect();

        let param_types = params
            .iter()
            .map(|param| format!("{}, ", param.ty))
            .collect();

        let param_names = params
            .iter()
            .map(|param| format!("{}, ", param.name))
            .collect();

        let result_type = result.unwrap_or("()".to_string());

        Self {
            rust_name,
            exported_name,
            imported_module,
            imported_name,
            param_full,
            param_types,
            param_names,
            result_type,
        }
    }
}
