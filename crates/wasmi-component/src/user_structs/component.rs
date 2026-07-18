use std::rc::Rc;

use wasmi::Engine;

use crate::lib_structs::{ComponentBuilder, FuncStorage};

#[derive(Debug, Clone)]
pub struct Component {
    pub(crate) core_module: wasmi::Module,
    pub(crate) imported_funcs: FuncStorage,
    pub(crate) exported_funcs: Rc<FuncStorage>,
}

impl Component {
    pub fn new(engine: &Engine, bytes: &[u8]) -> anyhow::Result<Self> {
        let builder = ComponentBuilder::new(bytes)?;

        let core_module = wasmi::Module::new(engine, builder.core_module()?)?;
        let imported_funcs = builder.imported_funcs();
        let exported_funcs = Rc::new(builder.exported_funcs());

        Ok(Self {
            core_module,
            imported_funcs,
            exported_funcs,
        })
    }
}
