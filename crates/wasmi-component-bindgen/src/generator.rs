use std::fmt::Write;

use heck::{ToSnakeCase, ToUpperCamelCase};

use crate::parse::ParsedWorld;

pub fn generate_wit(worlds: &[ParsedWorld], output: &mut String) {
    writeln!(
        output,
        concat!(
            "use wasmi_component::anyhow::{{Context, Result}};\n",
            "#[allow(unused)]\n",
            "use wasmi_component::wasmi::{{AsContext, AsContextMut, Caller, ",
            "FuncType, Linker, ValType}};\n",
            "#[allow(unused)]\n",
            "use wasmi_component::{{AsHostStorage, Component, HostResult, Lift, Lower, LowerVal, ",
            "MemoryAccessPre, TypedFunc, anyhow_result_to_wasmi}};\n",
        )
    )
    .unwrap();

    worlds
        .iter()
        .for_each(|world| generate_world(world, output));
}

fn generate_world(world: &ParsedWorld, output: &mut String) {
    let imports_name = format!("{}Imports", world.world_name.to_upper_camel_case());
    let exports_name = format!("{}Exports", world.world_name.to_upper_camel_case());

    writeln!(output, "pub trait {imports_name} {{").unwrap();
    world.imports.iter().for_each(|func| {
        writeln!(
            output,
            "  fn {}(&mut self, {}) -> HostResult<impl LowerVal<Target = {}> + 'static>;\n",
            func.rust_name, func.param_full, func.result_type
        )
        .unwrap();
    });
    writeln!(output, "}}").unwrap();
    writeln!(output).unwrap();

    writeln!(output, "#[allow(unused)]").unwrap();
    writeln!(output, "pub struct {exports_name} {{").unwrap();
    world.exports.iter().for_each(|func| {
        writeln!(
            output,
            "  pub {}: TypedFunc<({}), {}>,",
            func.rust_name, func.param_types, func.result_type
        )
        .unwrap();
    });
    writeln!(output, "}}").unwrap();
    writeln!(output).unwrap();

    writeln!(
        output,
        concat!(
            "pub fn instantiate_{}_world<D: AsHostStorage{}>",
            "(mut ctx: impl AsContextMut<Data = D>, component: &Component)",
            " -> Result<{}> {{",
        ),
        world.world_name.to_snake_case(),
        if !world.imports.is_empty() {
            format!(" + {imports_name}")
        } else {
            "".to_string()
        },
        exports_name
    )
    .unwrap();

    writeln!(
        output,
        concat!(
            "  #[allow(unused_mut)]\n",
            "  let mut linker = Linker::<D>::new(ctx.as_context().engine());\n",
            "  let memory_index = ctx",
            ".as_context_mut()",
            ".data_mut()",
            ".as_host_storage_mut()",
            ".next_memory_index();\n",
        )
    )
    .unwrap();

    world.imports.iter().for_each(|func| {
        writeln!(
            output,
            concat!(
                "    let mut params_ty = <({})>::imported_params();\n",
                "    let result_ty = <{}>::imported_result();\n",
                "    let has_external_result = result_ty.len() < <{}>::params_count();\n",
                "    if has_external_result {{\n",
                "      params_ty.push(ValType::I32);\n",
                "    }}\n",
            ),
            func.param_types, func.result_type, func.result_type
        )
        .unwrap();

        writeln!(
            output,
            concat!(
                "  linker.func_new(\"{}\", \"{}\", ",
                "FuncType::new(params_ty, result_ty), ",
                "move |mut caller, params, results| {{",
            ),
            func.imported_module, func.imported_name,
        )
        .unwrap();

        writeln!(
            output,
            concat!(
                "    let memory_pre = *caller.data().",
                "as_host_storage().get_memory(memory_index);\n",
                "    let (bytes, user_data) = memory_pre.memory.",
                "data_and_store_mut(caller.as_context_mut());\n",
                "\n",
                "    #[allow(unused)]\n",
                "    let params = anyhow_result_to_wasmi(<({})>::lift(params, bytes))?;\n",
                "    let res = user_data.{}({})?;",
                "\n",
                "    let mut memory_filled = memory_pre.fill(caller);\n",
                "    anyhow_result_to_wasmi(res.lower(results, &mut memory_filled))?;\n",
                "\n",
                "    Ok(())\n",
                "  }})?;\n"
            ),
            func.param_types, func.rust_name, func.param_args
        )
        .unwrap();
    });

    writeln!(
        output,
        concat!(
            "  let instance = linker.instantiate_and_start",
            "(ctx.as_context_mut(), &component.core_module)?;\n\n",
            "  let memory = instance.get_memory",
            "(ctx.as_context(), \"memory\").context(\"get memory\")?;\n",
            "  let cabi_realloc = instance.get_typed_func::<(i32, i32, i32, i32), i32>",
            "(ctx.as_context_mut(), \"cabi_realloc\")?;\n",
            "\n",
            "  let memory_pre = MemoryAccessPre::new(memory, cabi_realloc);\n",
            "  ctx.as_context_mut().data_mut().as_host_storage_mut()",
            ".insert_memory(memory_index, memory_pre);"
        )
    )
    .unwrap();

    world.exports.iter().for_each(|func| {
        writeln!(
            output,
            "  let module_func = instance.get_func(ctx.as_context_mut(), \"{}\").unwrap();",
            func.exported_name
        )
        .unwrap();
        writeln!(
            output,
            concat!(
                "  let cleanup_func = instance.get_typed_func::<i32, ()>",
                "(ctx.as_context_mut(), \"cabi_post_{}\").ok();"
            ),
            func.exported_name
        )
        .unwrap();
        writeln!(
            output,
            "  let {} = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);",
            func.rust_name
        )
        .unwrap();
        writeln!(output).unwrap();
    });

    writeln!(output, "  Ok({exports_name} {{").unwrap();
    world.exports.iter().for_each(|func| {
        writeln!(output, "      {},", func.rust_name).unwrap();
    });
    writeln!(output, "  }})").unwrap();

    writeln!(output, "}}").unwrap();
}
