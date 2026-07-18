use std::rc::Rc;

use wasmi::Engine;
use wasmi_component_parser::ValueType;

use crate::{ComponentBuilder, FuncStorage};

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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FuncSignature {
    /// All params squished into a single tuple.
    pub params: ValueType,
    pub result: ValueType,
}

impl FuncSignature {
    pub fn new(params: ValueType, result: ValueType) -> Self {
        Self { params, result }
    }
}

impl std::fmt::Display for FuncSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        if let ValueType::Tuple(tuple) = &self.params {
            for (index, field) in tuple.iter().enumerate() {
                if index > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", field)?;
            }
        } else {
            write!(f, "{}", &self.params)?;
        }

        write!(f, ") -> {}", self.result)
    }
}
