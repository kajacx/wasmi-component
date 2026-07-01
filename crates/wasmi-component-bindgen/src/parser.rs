use heck::{ToSnakeCase, ToUpperCamelCase};
use std::fmt::Write;
use wit_parser::{Function, Interface, Type, UnresolvedPackage, WorldKey};
use wit_parser::{World, WorldItem};

pub struct Parser {
    pkg: UnresolvedPackage,
}

impl Parser {
    pub fn new(pkg: UnresolvedPackage) -> Self {
        Self { pkg }
    }

    pub fn parse_wit(&self) -> String {
        let mut output = String::new();

        self.pkg
            .worlds
            .iter()
            .for_each(|(_, world)| self.generate_world(world, &mut output));

        output
    }

    fn generate_world(&self, world: &World, output: &mut String) {
        let exports_name = format!("{}Exports", world.name.to_upper_camel_case());
        let exported_funcs = self.parse_exported_world_functions(world);

        writeln!(
            output,
            concat!(
                "use wasmi_component::anyhow::{{Context, Result}};\n",
                "use wasmi_component::wasmi::{{AsContextMut, Linker}};\n",
                "use wasmi_component::{{Component, MemoryAccessPre, TypedFunc}};\n",
            )
        )
        .unwrap();

        writeln!(output, "#[allow(unused)]").unwrap();
        writeln!(output, "pub struct {exports_name} {{").unwrap();
        exported_funcs.iter().for_each(|func| {
            writeln!(
                output,
                "    pub {}: TypedFunc<{}, {}>,",
                func.field_name, func.params_type_str, func.result_type_str
            )
            .unwrap();
        });
        writeln!(output, "}}").unwrap();
        writeln!(output).unwrap();

        writeln!(
            output,
            concat!(
                "pub fn instantiate_{}_world",
                "(mut ctx: impl AsContextMut, component: &Component)",
                " -> Result<{}> {{",
            ),
            world.name.to_snake_case(),
            exports_name
        )
        .unwrap();

        writeln!(
            output,
            concat!(
                "    let linker = Linker::new(ctx.as_context().engine());\n",
                "    let instance = linker.instantiate_and_start(ctx.as_context_mut(), &component.core_module)?;\n"
            )
        )
        .unwrap();

        writeln!(
            output,
            concat!(
                "    let memory = instance.get_memory",
                "(ctx.as_context(), \"memory\").context(\"get memory\")?;\n",
                "    let cabi_realloc = instance.get_typed_func::<(i32, i32, i32, i32), i32>",
                "(ctx.as_context_mut(), \"cabi_realloc\")?;\n",
                "    let memory_pre = MemoryAccessPre::new(memory, cabi_realloc);\n",
            )
        )
        .unwrap();

        exported_funcs.iter().for_each(|func| {
            writeln!(
                output,
                "    let module_func = instance.get_func(ctx.as_context_mut(), \"{}\").unwrap();",
                func.core_export_name
            )
            .unwrap();
            writeln!(
                output,
                concat!(
                    "    let cleanup_func = instance.get_typed_func::<i32, ()>",
                    "(ctx.as_context_mut(), \"cabi_post_{}\").ok();"
                ),
                func.core_export_name
            )
            .unwrap();
            writeln!(
                output,
                "    let {} = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);",
                func.field_name
            )
            .unwrap();
            writeln!(output).unwrap();
        });

        writeln!(output, "    Ok({exports_name} {{").unwrap();
        exported_funcs.iter().for_each(|func| {
            writeln!(output, "        {},", func.field_name).unwrap();
        });
        writeln!(output, "    }})").unwrap();

        writeln!(output, "}}").unwrap();
    }

    fn parse_exported_world_functions<'a>(&'a self, world: &World) -> Vec<PreparedFunction> {
        world
            .exports
            .iter()
            .flat_map(|(key, value)| match value {
                WorldItem::Function(func) => vec![self.parse_function(func, key, None)],
                WorldItem::Interface { id, .. } => self
                    .parse_exported_interface_functions(key, self.pkg.interfaces.get(*id).unwrap()),
                _ => vec![],
            })
            .collect()
    }

    fn parse_exported_interface_functions(
        &self,
        key: &WorldKey,
        interface: &Interface,
    ) -> Vec<PreparedFunction> {
        interface
            .functions
            .values()
            .map(|func| self.parse_function(func, key, Some(interface)))
            .collect()
    }

    fn parse_function(
        &self,
        func: &Function,
        key: &WorldKey,
        interface: Option<&Interface>,
    ) -> PreparedFunction {
        let field_name = if let Some(name) = interface.and_then(|iface| iface.name.as_ref()) {
            format!("{}_{}", name.to_snake_case(), func.name.to_snake_case())
        } else {
            func.name.to_snake_case()
        };

        let interface_name = interface.and_then(|iface| iface.name.as_ref());
        let core_export_name = match (interface, interface_name) {
            (Some(_), Some(interface_name)) => {
                let namespace = &self.pkg.name.namespace;
                let pkg_name = &self.pkg.name.name;
                let version = if let Some(ver) = &self.pkg.name.version {
                    format!("@{ver}")
                } else {
                    "".to_string()
                };

                let func_name = &func.name;
                format!("{namespace}:{pkg_name}/{interface_name}{version}#{func_name}")
            }
            (Some(_), None) => {
                format!("{}#{}", key.clone().unwrap_name(), func.name)
            }
            (None, _) => {
                format!("{}", func.name)
            }
        };

        let params: Vec<_> = func
            .params
            .iter()
            .map(|param| PreparedType {
                wit_type: param.ty,
                wit_type_str: self.get_type_name(&param.ty),
            })
            .collect();

        let params_type_str = if params.len() == 1 {
            params[0].wit_type_str.clone()
        } else {
            let mut params_str = String::from("(");
            params.iter().for_each(|param| {
                params_str.push_str(&param.wit_type_str);
                params_str.push_str(", ");
            });
            params_str.push_str(")");
            params_str
        };

        let result = func.result.map(|result| PreparedType {
            wit_type: result,
            wit_type_str: self.get_type_name(&result),
        });

        let result_type_str = result
            .as_ref()
            .map_or_else(|| "()".to_string(), |result| result.wit_type_str.clone());

        PreparedFunction {
            field_name,
            core_export_name,
            params,
            params_type_str,
            result,
            result_type_str,
        }
    }

    fn get_type_name(&self, ty: &Type) -> String {
        match ty {
            Type::Bool => "bool".to_string(),
            Type::Char => "char".to_string(),
            Type::S8 => "i8".to_string(),
            Type::S16 => "i16".to_string(),
            Type::S32 => "i32".to_string(),
            Type::S64 => "i64".to_string(),
            Type::U8 => "u8".to_string(),
            Type::U16 => "u16".to_string(),
            Type::U32 => "u32".to_string(),
            Type::U64 => "u64".to_string(),
            Type::F32 => "f32".to_string(),
            Type::F64 => "f64".to_string(),
            Type::String => "String".to_string(),
            Type::ErrorContext => todo!(),
            Type::Id(_) => todo!(),
        }
    }
}

#[allow(unused)]
struct PreparedFunction {
    field_name: String,
    core_export_name: String,
    params: Vec<PreparedType>,
    params_type_str: String,
    result: Option<PreparedType>,
    result_type_str: String,
}

#[allow(unused)]
struct PreparedType {
    wit_type: Type,
    wit_type_str: String,
}
