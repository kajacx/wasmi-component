use std::fmt::Write;

use heck::{ToSnakeCase, ToUpperCamelCase};

use crate::parse::{ParsedWit, ParsedWorld};

pub fn generate_wit(wit: ParsedWit, output: &mut String) {
    writeln!(
        output,
        concat!(
            "use wasmi_component::anyhow::{{Context, Result}};\n",
            "#[allow(unused)]\n",
            "use wasmi_component::wasmi::{{AsContext, AsContextMut, Caller, ",
            "FuncType, Linker, ValType}};\n",
            "#[allow(unused)]\n",
            "use wasmi_component::{{Borrow, Component, CompValue, HostResult, ",
            "LowerVal, MemoryAccessPre, Own, StoreData, TypedFunc, anyhow_result_to_wasmi}};\n",
            "#[allow(unused)]\n",
            "use wasmi_component::wasi_p2::resources::*;\n"
        )
    )
    .unwrap();

    // wit.types
    //     .iter()
    //     .for_each(|ty| writeln!(output, "{ty}").unwrap());

    wit.worlds
        .iter()
        .for_each(|world| generate_world(world, output));
}

fn generate_world(world: &ParsedWorld, output: &mut String) {
    let imports_name = format!("{}Imports", world.world_name.to_upper_camel_case());
    let exports_name = format!("{}Exports", world.world_name.to_upper_camel_case());

    writeln!(output, "#[allow(unused)]").unwrap();
    writeln!(output, "pub trait {imports_name} {{").unwrap();
    world.imports.iter().for_each(|func| {
        writeln!(
            output,
            "  fn {}(&mut self, {}) -> HostResult<{}>;\n",
            func.rust_name(),
            func.host_params_full(),
            func.host_return_type()
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
            func.rust_name(),
            func.param_types(),
            func.result
        )
        .unwrap();
    });
    writeln!(output, "}}").unwrap();
    writeln!(output).unwrap();

    let imports_bound = if !world.imports.is_empty() {
        format!(": {imports_name}")
    } else {
        "".to_string()
    };

    writeln!(
        output,
        concat!(
            "#[allow(unused)]\n",
            "pub fn add_{}_to_linker<D{}>",
            "(mut ctx: impl AsContextMut<Data = StoreData<D>>,",
            " linker: &mut Linker<StoreData<D>>, memory_index: usize)",
            " -> Result<()> {{"
        ),
        world.world_name.to_snake_case(),
        imports_bound
    )
    .unwrap();

    world.imports.iter().for_each(|func| {
        writeln!(
            output,
            concat!(
                "    let mut params_ty = <({})>::arg_types();\n",
                "    let mut result_ty = <{}>::arg_types();\n",
                "    let has_external_result = result_ty.len() > 1;\n",
                "    if has_external_result {{\n",
                "      params_ty.push(ValType::I32);\n",
                "      result_ty.clear();\n",
                "    }}\n",
            ),
            func.param_types(),
            func.result
        )
        .unwrap();

        writeln!(
            output,
            concat!(
                "  linker.func_new(\"{}\", \"{}\", ",
                "FuncType::new(params_ty, result_ty), ",
                "move |mut caller, params, results| {{",
            ),
            func.module_name.as_deref().unwrap_or("$root"),
            func.func_name,
        )
        .unwrap();

        writeln!(
            output,
            concat!(
                "    let memory_pre = *caller.data().get_memory(memory_index);\n",
                "    let (bytes, user_data) = memory_pre.memory.",
                "data_and_store_mut(caller.as_context_mut());\n",
                "\n",
                "    let params_slice = if has_external_result {{\n",
                "        &params[0..(params.len() - 1)]\n",
                "    }} else {{\n",
                "        params\n",
                "    }};\n",
                "\n",
                "    #[allow(unused)]\n",
                "    let args = anyhow_result_to_wasmi(<({})>::lift_args(params_slice, bytes))?;\n",
                "    let res = user_data.data_mut().{}({})?;",
                "\n",
                "    let mut memory_filled = memory_pre.fill(caller);\n",
                "\n",
                "    if has_external_result {{\n",
                "      let address = params[params.len() - 1].i32().unwrap() as usize;\n",
                "      let range = address..(address + <{}>::byte_size());\n",
                "      anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;\n",
                "    }} else {{\n",
                "      anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;\n",
                "    }}\n",
                "\n",
                "    Ok(())\n",
                "  }})?;\n"
            ),
            func.param_types(),
            func.rust_name(),
            func.param_args(),
            func.result
        )
        .unwrap();
    });

    writeln!(output, "  Ok(())\n}}\n").unwrap();

    writeln!(
        output,
        concat!(
            "#[allow(unused)]\n",
            "pub fn instantiate_{}_world<D{}>",
            "(mut ctx: impl AsContextMut<Data = StoreData<D>>, component: &Component)",
            " -> Result<{}> {{",
        ),
        world.world_name.to_snake_case(),
        imports_bound,
        exports_name
    )
    .unwrap();

    writeln!(
        output,
        concat!(
            "  #[allow(unused_mut)]\n",
            "  let mut linker = Linker::<StoreData<D>>::new(ctx.as_context().engine());\n",
            "  let memory_index = ctx",
            ".as_context_mut()",
            ".data_mut()",
            ".next_memory_index();\n",
        )
    )
    .unwrap();

    writeln!(
        output,
        "add_{}_to_linker(ctx.as_context_mut(), &mut linker, memory_index)?;\n",
        world.world_name.to_snake_case()
    )
    .unwrap();

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
            "  ctx.as_context_mut().data_mut().insert_memory(memory_index, memory_pre);\n"
        )
    )
    .unwrap();

    world.exports.iter().for_each(|func| {
        writeln!(
            output,
            "  let module_func = instance.get_func(ctx.as_context_mut(), \"{}\").unwrap();",
            func.exported_name()
        )
        .unwrap();
        writeln!(
            output,
            concat!(
                "  let cleanup_func = instance.get_typed_func::<i32, ()>",
                "(ctx.as_context_mut(), \"cabi_post_{}\").ok();"
            ),
            func.exported_name()
        )
        .unwrap();
        writeln!(
            output,
            "  let {} = TypedFunc::new(memory_pre.clone(), module_func, cleanup_func);",
            func.rust_name()
        )
        .unwrap();
        writeln!(output).unwrap();
    });

    writeln!(output, "  Ok({exports_name} {{").unwrap();
    world.exports.iter().for_each(|func| {
        writeln!(output, "      {},", func.rust_name()).unwrap();
    });
    writeln!(output, "  }})").unwrap();

    writeln!(output, "}}").unwrap();
}
