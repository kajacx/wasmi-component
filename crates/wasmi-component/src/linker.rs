use crate::*;

#[derive(Debug, Default)]
pub struct Linker {}

impl Linker {
    pub fn instantiate<T>(
        &self,
        _ctx: &mut Store<T>,
        _component: &Component,
    ) -> Result<(), wasmi::Error> {
        Ok(())
    }
}
