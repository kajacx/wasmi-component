use std::fmt::Write;

use heck::{ToSnakeCase, ToUpperCamelCase};

use crate::parse::ParsedWorld;

pub fn generate_wit(worlds: &[ParsedWorld], output: &mut String) {
    writeln!(
        output,
        concat!(
            "use wasmi_component::anyhow::{{Context, Result}};\n",
            "#[allow(unused)]\n",
            "use wasmi_component::wasmi::{{AsContextMut, Caller, Linker}};\n",
            "#[allow(unused)]\n",
            "use wasmi_component::{{Component, HostResult, ",
            "MemoryAccessPre, TypedFunc, WitString}};\n",
        )
    )
    .unwrap();

    worlds
        .iter()
        .for_each(|world| generate_world(world, output));
}

fn generate_world(world: &ParsedWorld, output: &mut String) {
    let exports_name = format!("{}Exports", world.world_name.to_upper_camel_case());
    let imports_name = format!("{}Imports", world.world_name.to_upper_camel_case());

    writeln!(output, "#[allow(unused)]").unwrap();
    writeln!(output, "pub struct {exports_name} {{").unwrap();
    world.exports.iter().for_each(|func| {
        writeln!(
            output,
            "    pub {}: TypedFunc<({}), {}>,",
            func.rust_name, func.param_types, func.result_type
        )
        .unwrap();
    });
    writeln!(output, "}}").unwrap();
    writeln!(output).unwrap();

    writeln!(output, "pub trait {imports_name} {{").unwrap();
    world.imports.iter().for_each(|func| {
        writeln!(
            output,
            "    fn {}(&mut self, {}) -> HostResult<{}>;",
            func.rust_name, func.param_full, func.result_type
        )
        .unwrap();
    });
    writeln!(output, "}}").unwrap();
    writeln!(output).unwrap();

    writeln!(
        output,
        concat!(
            "pub fn instantiate_{}_world<D{}>",
            "(mut ctx: impl AsContextMut<Data = D>, component: &Component)",
            " -> Result<{}> {{",
        ),
        world.world_name.to_snake_case(),
        if !world.imports.is_empty() {
            format!(": {imports_name}")
        } else {
            "".to_string()
        },
        exports_name
    )
    .unwrap();

    writeln!(
        output,
        concat!(
            "    #[allow(unused_mut)]\n",
            "    let mut linker = Linker::new(ctx.as_context().engine());\n",
        )
    )
    .unwrap();

    world.imports.iter().for_each(|func| {
        writeln!(
            output,
            concat!(
                "    linker.func_wrap(\"{}\", \"{}\", ",
                "|mut caller: Caller<D>, {}| caller.data_mut().{}({})",
                ")?;"
            ),
            func.imported_module,
            func.imported_name,
            func.param_full,
            func.rust_name,
            func.param_names,
        )
        .unwrap();
    });

    writeln!(
        output,
        concat!(
            "    let instance = linker.instantiate_and_start",
            "(ctx.as_context_mut(), &component.core_module)?;\n\n",
            "    let memory = instance.get_memory",
            "(ctx.as_context(), \"memory\").context(\"get memory\")?;\n",
            "    let cabi_realloc = instance.get_typed_func::<(i32, i32, i32, i32), i32>",
            "(ctx.as_context_mut(), \"cabi_realloc\")?;\n",
            "    let memory_pre = MemoryAccessPre::new(memory, cabi_realloc);\n",
        )
    )
    .unwrap();

    world.exports.iter().for_each(|func| {
        writeln!(
            output,
            "    let module_func = instance.get_func(ctx.as_context_mut(), \"{}\").unwrap();",
            func.exported_name
        )
        .unwrap();
        writeln!(
            output,
            concat!(
                "    let cleanup_func = instance.get_typed_func::<i32, ()>",
                "(ctx.as_context_mut(), \"cabi_post_{}\").ok();"
            ),
            func.exported_name
        )
        .unwrap();
        writeln!(
            output,
            "    let {} = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);",
            func.rust_name
        )
        .unwrap();
        writeln!(output).unwrap();
    });

    writeln!(output, "    Ok({exports_name} {{").unwrap();
    world.exports.iter().for_each(|func| {
        writeln!(output, "        {},", func.rust_name).unwrap();
    });
    writeln!(output, "    }})").unwrap();

    writeln!(output, "}}").unwrap();
}
