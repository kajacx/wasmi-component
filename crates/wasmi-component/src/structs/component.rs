use wasmi::Engine;
use wasmparser::{Parser, Payload};

use crate::*;

#[derive(Debug)]
pub struct Component {
    pub(crate) core_module: wasmi::Module,
}

impl Component {
    pub fn new(engine: &Engine, bytes: &[u8]) -> Result<Self, wasmi::Error> {
        let parser = Parser::new(0);

        let mut modules = Vec::with_capacity(4);

        for payload in parser.parse_all(bytes) {
            match payload.map_err(|err| wasmi::Error::new(err.message()))? {
                Payload::ModuleSection {
                    unchecked_range, ..
                } => {
                    let module_bytes = &bytes[unchecked_range];
                    let module = wasmi::Module::new(engine, module_bytes)?;
                    modules.push(module);
                }
                _ => {}
            }
        }

        if modules.len() >= 1 {
            Ok(Self {
                core_module: modules.remove(0),
            })
        } else {
            Err(wasmi::Error::new("component did contain any core modules"))
        }
    }
}
