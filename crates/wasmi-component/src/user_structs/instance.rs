use std::rc::Rc;

use anyhow::{Context, ensure};
use wasmi::AsContext;
use wasmi_component_parser::FuncIdentifier;

use crate::lib_structs::{FuncSignature, FuncStorage, MemoryAccessPre};
use crate::{ComponentValue, TypedFunc, UntypedFunc};

#[derive(Debug, Clone)]
pub struct Instance {
    instance: wasmi::Instance,
    exports: Rc<FuncStorage>,
    memory_pre: MemoryAccessPre,
}

impl Instance {
    pub(crate) fn new(
        instance: wasmi::Instance,
        exports: Rc<FuncStorage>,
        memory_pre: MemoryAccessPre,
    ) -> Self {
        Self {
            instance,
            exports,
            memory_pre,
        }
    }

    pub fn get_typed_func<Params: ComponentValue, Results: ComponentValue>(
        &self,
        ctx: impl AsContext,
        module: impl Into<String>,
        name: impl Into<String>,
    ) -> anyhow::Result<TypedFunc<Params, Results>> {
        let ident = FuncIdentifier::new(module.into(), name.into());
        let exported_name = ident.exported_name();

        self.exports.verify_export(
            &ident,
            &FuncSignature::from_grouped(Params::value_type(), Results::value_type()),
        )?;

        let module_func = self
            .instance
            .get_func(ctx.as_context(), &exported_name)
            .with_context(|| format!("exported function {} not found.", exported_name))?;

        let ty = module_func.ty(ctx.as_context());

        let mut result_types = Results::arg_types();
        if result_types.len() > 1 {
            result_types.clear();
            result_types.push(wasmi::ValType::I32);
        }

        ensure!(
            ty.params() == Params::arg_types() && ty.results() == result_types,
            "Incorrect signature for exported function {}, expected {:?} -> {:?}, but got {:?} -> {:?} instead",
            exported_name,
            Params::arg_types(),
            result_types,
            ty.params(),
            ty.results()
        );

        let cleanup_func = self
            .instance
            .get_typed_func::<i32, ()>(ctx.as_context(), &format!("cabi_post_{}", exported_name))
            .ok();

        if cleanup_func.is_some() {
            ensure!(
                ty.params() != &[wasmi::ValType::I32],
                "exported function {} has a cleanup function, but doesn't take a single i32",
                ident
            );
        }

        Ok(TypedFunc::new(
            self.memory_pre.clone(),
            module_func,
            cleanup_func,
        ))
    }

    pub fn get_untyped_func<Params: ComponentValue, Results: ComponentValue>(
        &self,
        ctx: impl AsContext,
        module: impl Into<String>,
        name: impl Into<String>,
    ) -> anyhow::Result<UntypedFunc> {
        let ident = FuncIdentifier::new(module.into(), name.into());
        let exported_name = ident.exported_name();

        let exported_func = self.exports.get(&ident).with_context(|| {
            format!("exported function {} not found in component exports", ident)
        })?;

        let module_func = self
            .instance
            .get_func(ctx.as_context(), &exported_name)
            .with_context(|| {
                format!(
                    "exported function {} not found in core wasm module",
                    exported_name
                )
            })?;

        let cleanup_func = self
            .instance
            .get_typed_func::<i32, ()>(ctx.as_context(), &format!("cabi_post_{}", exported_name))
            .ok();

        if cleanup_func.is_some() {
            let ty = module_func.ty(ctx.as_context());
            ensure!(
                ty.params() != &[wasmi::ValType::I32],
                "exported function {} has a cleanup function, but doesn't take a single i32",
                ident
            );
        }

        Ok(UntypedFunc::new(
            self.memory_pre.clone(),
            module_func,
            cleanup_func,
            ident,
            exported_func.clone(),
        ))
    }
}
