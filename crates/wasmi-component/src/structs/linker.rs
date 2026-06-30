use crate::*;

#[derive(Debug)]
pub struct Linker<T> {
    pub linker: wasmi::Linker<T>,
}

impl<T> Linker<T> {
    pub fn new(engine: &Engine) -> Self {
        Self {
            linker: wasmi::Linker::new(engine),
        }
    }

    pub fn instantiate(
        &self,
        ctx: &mut Store<T>,
        component: &Component,
    ) -> Result<Instance, wasmi::Error> {
        let instance = self
            .linker
            .instantiate_and_start(ctx, &component.core_module)?;

        Ok(Instance { instance })
    }
}
