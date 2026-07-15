use std::fmt::Write;

use heck::{ToSnakeCase, ToUpperCamelCase};
use wasmi_component_parser::{ParsedWit, ParsedWorld, ValueType};

use crate::{
    func_helpers::{
        exported_name, param_names_as_args, param_types_canon, params_full_lift, params_full_lower,
        params_indexes,
    },
    type_helpers::{canonical_name, liftable_type, rust_snake_case},
};

pub struct Generator {}

impl Generator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_wit(&self, wit: ParsedWit) -> String {
        let mut output = String::new();

        writeln!(
            output,
            concat!(
                "use wasmi_component::anyhow::Result;\n",
                "#[allow(unused)]\n",
                "use wasmi_component::wasmi::{{AsContext, AsContextMut, errors::LinkerError}};\n",
                "#[allow(unused)]\n",
                "use wasmi_component::{{CallResult, Component, ComponentValue, HostResult, ",
                "Linker, ListAccessor, LowerValue, StoreData, TypedFunc}};\n",
            ),
        )
        .unwrap();

        for ty in &wit.types {
            self.write_type(ty, &mut output);
        }

        for world in &wit.worlds {
            self.generate_world(world, &mut output);
        }

        output
    }

    fn write_type(&self, ty: &ValueType, output: &mut String) {
        writeln!(output, "#[allow(unused)]").unwrap();
        writeln!(
            output,
            "#[derive(Debug, Clone, PartialEq, PartialOrd, ComponentValue)]"
        )
        .unwrap();

        match ty {
            ValueType::Record { name, fields } => {
                writeln!(output, "pub struct {} {{", name.to_upper_camel_case()).unwrap();

                fields.iter().for_each(|(name, ty)| {
                    writeln!(
                        output,
                        "    pub {}: {},",
                        rust_snake_case(name),
                        canonical_name(ty)
                    )
                    .unwrap();
                });

                writeln!(output, "}}\n").unwrap();
            }
            ValueType::Variant { name, cases } => {
                writeln!(output, "pub enum {} {{", name.to_upper_camel_case()).unwrap();

                cases.iter().for_each(|(name, ty)| {
                    writeln!(
                        output,
                        "    {}{},",
                        name.to_upper_camel_case(),
                        if let Some(ty) = ty {
                            format!("({})", canonical_name(ty))
                        } else {
                            "".to_string()
                        }
                    )
                    .unwrap();
                });

                writeln!(output, "}}\n").unwrap();
            }
            other => {
                panic!("expected a record or a variant, got {:?} instead", other);
            }
        }
    }

    fn generate_world(&self, world: &ParsedWorld, output: &mut String) {
        self.generate_imports_trait(world, output);

        self.generate_exports_struct(world, output);

        self.generate_linker(world, output);

        self.generate_instantiate(world, output);
    }

    fn generate_imports_trait(&self, world: &ParsedWorld, output: &mut String) {
        writeln!(output, "#[allow(unused)]").unwrap();
        writeln!(
            output,
            "pub trait {}Imports {{",
            world.name.to_upper_camel_case()
        )
        .unwrap();

        world.imports.iter().for_each(|func| {
            writeln!(
                output,
                "    fn {}(&mut self, {}) -> HostResult<{}>;\n",
                rust_snake_case(&func.func_name),
                params_full_lift(func),
                canonical_name(&func.result)
            )
            .unwrap();
        });

        writeln!(output, "}}\n").unwrap();
    }

    fn generate_exports_struct(&self, world: &ParsedWorld, output: &mut String) {
        writeln!(output, "#[allow(unused)]").unwrap();
        writeln!(
            output,
            "pub struct {}Exports {{",
            world.name.to_upper_camel_case()
        )
        .unwrap();

        world.exports.iter().for_each(|func| {
            writeln!(
                output,
                "    pub {}: TypedFunc<({}), {}>,",
                rust_snake_case(&func.func_name),
                param_types_canon(func),
                canonical_name(&func.result)
            )
            .unwrap();
        });

        writeln!(output, "}}\n").unwrap();

        writeln!(output, "#[allow(unused)]").unwrap();
        writeln!(
            output,
            "impl {}Exports {{",
            world.name.to_upper_camel_case()
        )
        .unwrap();

        world.exports.iter().for_each(|func| {
            writeln!(
                output,
                concat!(
                    "    pub fn call_{}<T>",
                    "(&self, ctx: impl AsContextMut<Data = StoreData<T>>, {}) ",
                    "-> CallResult<{}> {{\n",
                    "        self.{}.call(ctx, ({}))\n",
                    "    }}\n"
                ),
                rust_snake_case(&func.func_name),
                params_full_lower(func),
                canonical_name(&func.result),
                rust_snake_case(&func.func_name),
                param_names_as_args(func),
            )
            .unwrap();

            writeln!(
                output,
                concat!(
                    "    pub fn call_{}_with_results<T, R>(&self, ",
                    "ctx: impl AsContextMut<Data = StoreData<T>>, {}",
                    "callback: impl FnOnce(&mut T, {}) -> R)-> CallResult<R> {{\n",
                    "        self.{}.call_with_results(ctx, ({}), callback)\n",
                    "    }}\n"
                ),
                rust_snake_case(&func.func_name),
                params_full_lower(func),
                liftable_type(&func.result),
                rust_snake_case(&func.func_name),
                param_names_as_args(func),
            )
            .unwrap();
        });

        writeln!(output, "}}\n").unwrap();
    }

    fn generate_linker(&self, world: &ParsedWorld, output: &mut String) {
        writeln!(
            output,
            concat!(
                "#[allow(unused)]\n",
                "pub fn add_{}_to_linker<T{}>(linker: &mut Linker<T>) -> Result<(), LinkerError> {{"
            ),
            world.name.to_snake_case(),
            if world.imports.is_empty() {
                String::new()
            } else {
                format!(": {}Imports", world.name.to_upper_camel_case())
            }
        )
        .unwrap();

        world.imports.iter().for_each(|func| {
            writeln!(
                output,
                concat!(
                    "    linker.func_new::<({}), {}>",
                    "(\"{}\", \"{}\", |host_data, params| host_data.{}({}))?;\n"
                ),
                param_types_canon(func),
                canonical_name(&func.result),
                func.module_name.as_deref().unwrap_or("$root"),
                func.func_name,
                rust_snake_case(&func.func_name),
                params_indexes(func)
            )
            .unwrap();
        });

        writeln!(output, "  Ok(())\n}}\n").unwrap();
    }

    fn generate_instantiate(&self, world: &ParsedWorld, output: &mut String) {
        writeln!(
            output,
            concat!(
                "#[allow(unused)]\n",
                "pub fn instantiate_{}_world<T>",
                "(mut ctx: impl AsContextMut<Data = StoreData<T>>, ",
                "linker: &Linker<T>, component: &Component)",
                " -> Result<{}Exports, > {{",
            ),
            world.name.to_snake_case(),
            world.name.to_upper_camel_case()
        )
        .unwrap();

        writeln!(
            output,
            "    let instance = linker.instantiate(ctx.as_context_mut(), &component)?;\n",
        )
        .unwrap();

        world.exports.iter().for_each(|func| {
            writeln!(
                output,
                "    let {} = instance.get_typed_func(ctx.as_context(), \"{}\")?;",
                rust_snake_case(&func.func_name),
                exported_name(func)
            )
            .unwrap();
            writeln!(output).unwrap();
        });

        writeln!(
            output,
            "    Ok({}Exports {{",
            world.name.to_upper_camel_case()
        )
        .unwrap();
        world.exports.iter().for_each(|func| {
            writeln!(output, "        {},", rust_snake_case(&func.func_name)).unwrap();
        });
        writeln!(output, "    }})").unwrap();

        writeln!(output, "}}").unwrap();
    }
}
