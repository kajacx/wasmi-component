use wasmi::AsContextMut;

use crate::{Lift, LowerList};

pub struct TypedFunc<Params: LowerList, Results: Lift> {
    inner: wasmi::TypedFunc<Params::CoreType, Results::CoreType>,
    post_return: Option<wasmi::TypedFunc<i32, ()>>,
}

impl<Params: LowerList, Results: Lift> TypedFunc<Params, Results> {
    pub fn new(
        inner: wasmi::TypedFunc<Params::CoreType, Results::CoreType>,
        post_return: Option<wasmi::TypedFunc<i32, ()>>,
    ) -> Self {
        Self { inner, post_return }
    }

    pub fn call(
        &self,
        mut store: impl AsContextMut,
        params: Params,
    ) -> Result<Results, wasmi::Error> {
        let result = self.inner.call(store.as_context_mut(), params.lower())?;
        let result_addr = Results::as_address(&result);

        let lifted = Results::lift(result);

        if let Some(post_return) = self.post_return {
            let address = result_addr.expect("TODO: cleanup BUT no not i32 return");
            post_return.call(store, address)?;
        }

        Ok(lifted)
    }
}
