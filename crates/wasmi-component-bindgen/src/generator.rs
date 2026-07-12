use std::fmt::Write;

use heck::ToSnakeCase;

use crate::parse::{ParsedWit, ParsedWorld};

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

        wit.types
            .iter()
            .for_each(|ty| writeln!(output, "{ty}").unwrap());

        wit.worlds
            .iter()
            .for_each(|world| self.generate_world(world, &mut output));

        output
    }

    fn generate_world(&self, world: &ParsedWorld, output: &mut String) {
        self.generate_imports_trait(world, output);

        self.generate_exports_struct(world, output);

        self.generate_linker(world, output);

        self.generate_instantiate(world, output);
    }

    fn generate_imports_trait(&self, world: &ParsedWorld, output: &mut String) {
        writeln!(output, "#[allow(unused)]").unwrap();
        writeln!(output, "pub trait {} {{", world.imports_name).unwrap();

        world.imports.iter().for_each(|func| {
            writeln!(
                output,
                "  fn {}(&mut self, {}) -> HostResult<{}>;\n",
                func.rust_name(),
                func.params_full_lift(),
                func.result.canon
            )
            .unwrap();
        });

        writeln!(output, "}}\n").unwrap();
    }

    fn generate_exports_struct(&self, world: &ParsedWorld, output: &mut String) {
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

        writeln!(output, "}}\n").unwrap();

        writeln!(output, "#[allow(unused)]").unwrap();
        writeln!(output, "impl {} {{", world.exports_name).unwrap();

        world.exports.iter().for_each(|func| {
            writeln!(
                output,
                concat!(
                    "  pub fn call_{}<T>(&self, ctx: impl AsContextMut<Data = StoreData<T>>, {}) ",
                    "-> CallResult<{}> {{\n",
                    "    self.{}.call(ctx, ({}))\n",
                    "  }}\n"
                ),
                func.rust_name(),
                func.params_full_lower(),
                func.result.canon,
                func.rust_name(),
                func.param_names(),
            )
            .unwrap();

            writeln!(
                output,
                concat!(
                    "  pub fn call_{}_with_results<T, R>(&self, ",
                    "ctx: impl AsContextMut<Data = StoreData<T>>, {}",
                    "callback: impl FnOnce({}) -> R)-> CallResult<R> {{\n",
                    "    self.{}.call_with_results(ctx, ({}), callback)\n",
                    "  }}\n"
                ),
                func.rust_name(),
                func.params_full_lower(),
                func.result.lift,
                func.rust_name(),
                func.param_names(),
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
            world.world_name.to_snake_case(),
            world.imports_bound
        )
        .unwrap();

        world.imports.iter().for_each(|func| {
            writeln!(
                output,
                concat!(
                    "  linker.func_new::<({}), {}>",
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
                " -> Result<{}, > {{",
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
