use std::fmt::Write;

use heck::{ToSnakeCase, ToUpperCamelCase};

use crate::parse::{LowerArg, ParsedWit, ParsedWorld};

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
                "use wasmi_component::wasmi::{{AsContext, AsContextMut}};\n",
                "#[allow(unused)]\n",
                "use wasmi_component::{{Borrow, Component, ComponentValue, HostResult, Linker, ",
                "ListAccessor, LowerVal, Own, StoreData, TypedFunc}};\n",
                "#[allow(unused)]\n",
                "use wasmi_component::wasi_p2::{{add_wasi_p2_to_linker, resources::*}};\n"
            ),
        )
        .unwrap();

        wit.worlds
            .iter()
            .for_each(|world| self.generate_world(world, &mut output));

        output
    }

    fn generate_world(&self, world: &ParsedWorld, output: &mut String) {
        writeln!(output, "#[allow(unused)]").unwrap();
        writeln!(output, "pub trait {} {{", world.imports_name).unwrap();

        world.imports.iter().for_each(|func| {
            if matches!(func.result.lower, LowerArg::LowerVal) {
                writeln!(
                    output,
                    "  type {}Return: LowerVal<{}> + 'static;\n",
                    func.func_name.to_upper_camel_case(),
                    func.result.canon
                )
                .unwrap();
            }

            writeln!(
                output,
                "  fn {}(&mut self, {}) -> HostResult<{}>;\n",
                func.rust_name(),
                func.params_full_lift(),
                func.host_return_type()
            )
            .unwrap();
        });

        writeln!(output, "}}").unwrap();
        writeln!(output).unwrap();

        writeln!(output, "#[allow(unused)]").unwrap();
        writeln!(output, "pub struct {} {{", world.exports_name).unwrap();
        world.exports.iter().for_each(|func| {
            writeln!(
                output,
                "  pub {}: TypedFunc<({}), {}>,",
                func.rust_name(),
                func.param_types_canon(),
                func.result.canon
            )
            .unwrap();
        });
        writeln!(output, "}}").unwrap();
        writeln!(output).unwrap();

        self.generate_linker(world, output);

        self.generate_instantiate(world, output);
    }

    fn generate_linker(&self, world: &ParsedWorld, output: &mut String) {
        writeln!(
            output,
            concat!(
                "#[allow(unused)]\n",
                "pub fn add_{}_to_linker<T{}>(linker: &mut Linker<T>) -> Result<()> {{"
            ),
            world.world_name.to_snake_case(),
            world.imports_bound
        )
        .unwrap();

        world.imports.iter().for_each(|func| {
            writeln!(
                output,
                concat!(
                    "  linker.func_new::<({}), {}, _>",
                    "(\"{}\", \"{}\", |host_data, params| host_data.{}({}))?;\n"
                ),
                func.param_types_canon(),
                func.result.canon,
                func.module_name.as_deref().unwrap_or("$root"),
                func.func_name,
                func.rust_name(),
                func.param_arg_indexes()
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
                " -> Result<{}> {{",
            ),
            world.world_name.to_snake_case(),
            world.exports_name
        )
        .unwrap();

        writeln!(
            output,
            "  let instance = linker.instantiate(ctx.as_context_mut(), &component)?;\n",
        )
        .unwrap();

        world.exports.iter().for_each(|func| {
            writeln!(
                output,
                "  let {} = instance.get_typed_func(ctx.as_context(), \"{}\")?;",
                func.rust_name(),
                func.exported_name()
            )
            .unwrap();
            writeln!(output).unwrap();
        });

        writeln!(output, "  Ok({} {{", world.exports_name).unwrap();
        world.exports.iter().for_each(|func| {
            writeln!(output, "      {},", func.rust_name()).unwrap();
        });
        writeln!(output, "  }})").unwrap();

        writeln!(output, "}}").unwrap();
    }
}
