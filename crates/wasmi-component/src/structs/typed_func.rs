use wasmi::{AsContextMut, Memory};

use crate::{Lift, LowerList};

pub struct TypedFunc<Params: LowerList, Results: Lift> {
    memory: Memory,
    inner: wasmi::TypedFunc<Params::CoreType, Results::CoreType>,
    post_return: Option<wasmi::TypedFunc<i32, ()>>,
}

impl<Params: LowerList, Results: Lift> TypedFunc<Params, Results> {
    pub fn new(
        memory: Memory,
        inner: wasmi::TypedFunc<Params::CoreType, Results::CoreType>,
        post_return: Option<wasmi::TypedFunc<i32, ()>>,
    ) -> Self {
        Self {
            memory,
            inner,
            post_return,
        }
    }

    pub fn call(
        &self,
        mut ctx: impl AsContextMut,
        params: Params,
    ) -> Result<Results, wasmi::Error> {
        let result = self.inner.call(ctx.as_context_mut(), params.lower())?;
        let result_addr = Results::as_address(&result);

        let lifted = Results::lift(result, ctx.as_context(), &self.memory);

        if let Some(post_return) = self.post_return {
            let address = result_addr.expect("TODO: cleanup BUT no not i32 return");
            post_return.call(ctx, address)?;
        }

        Ok(lifted)
    }
}
