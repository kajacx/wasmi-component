use anyhow::{Context, Result, ensure};
use wasmi::AsContext;

use crate::{ComponentValue, MemoryAccessPre, TypedFunc};

pub struct Instance {
    instance: wasmi::Instance,
    memory_pre: MemoryAccessPre,
}

impl Instance {
    pub(crate) fn new(instance: wasmi::Instance, memory_pre: MemoryAccessPre) -> Self {
        Self {
            instance,
            memory_pre,
        }
    }

    pub fn get_typed_func<Params: ComponentValue, Results: ComponentValue>(
        &self,
        ctx: impl AsContext,
        name: &str,
    ) -> Result<TypedFunc<Params, Results>> {
        let module_func = self
            .instance
            .get_func(ctx.as_context(), name)
            .with_context(|| format!("Exported function {name} not found."))?;

        let ty = module_func.ty(ctx.as_context());

        let mut result_types = Results::arg_types();
        if result_types.len() > 1 {
            result_types.clear();
            result_types.push(wasmi::ValType::I32);
        }

        ensure!(
            ty.params() == Params::arg_types() && ty.results() == result_types,
            "Incorrect signature for exported function {name}, expected {:?} -> {:?}, but got {:?} -> {:?} instead",
            Params::arg_types(),
            result_types,
            ty.params(),
            ty.results()
        );

        let cleanup_func = self
            .instance
            .get_typed_func::<i32, ()>(ctx.as_context(), &format!("cabi_post_{name}"))
            .ok();

        Ok(TypedFunc::new(
            self.memory_pre.clone(),
            module_func,
            cleanup_func,
        ))
    }
}
