use std::collections::HashMap;

use wasmi::Engine;
use wasmi_component_parser::ValueType;

use crate::ComponentBuilder;

#[derive(Debug, Clone)]
pub struct Component {
    pub(crate) core_module: wasmi::Module,

    pub(crate) imported_funcs: HashMap<String, FuncSignature>,

    pub(crate) exported_funcs: HashMap<String, FuncSignature>,
}

impl Component {
    pub fn new(engine: &Engine, bytes: &[u8]) -> anyhow::Result<Self> {
        let builder = ComponentBuilder::new(bytes)?;

        let core_module = wasmi::Module::new(engine, builder.core_module()?)?;
        let imported_funcs = builder.imported_funcs()?;
        let exported_funcs = builder.exported_funcs()?;

        println!("IMPORTED: {:?}", imported_funcs);
        println!("EXPORTED: {:?}", exported_funcs);

        Ok(Self {
            core_module,
            imported_funcs,
            exported_funcs,
        })
    }
}

#[derive(Debug, Clone)]
pub struct FuncSignature {
    pub params: Vec<(String, ValueType)>,
    pub result: ValueType,
}
