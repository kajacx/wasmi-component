use wasmparser::{Parser, Payload};

use crate::*;

#[derive(Debug)]
pub struct Component {
    core_module: wasmi::Module,
}

impl Component {
    pub fn new(engine: &Engine, bytes: &[u8]) -> Result<Self, wasmi::Error> {
        let parser = Parser::new(0);

        let mut modules = Vec::with_capacity(4);

        for payload in parser.parse_all(bytes) {
            // println!("reading: {:?}", payload);
            match payload.expect("TODO:") {
                Payload::ModuleSection {
                    unchecked_range, ..
                } => {
                    let module_bytes = &bytes[unchecked_range];

                    let module_name = format!("../modules/module{}.core.wasm", modules.len());
                    let mut file = std::fs::File::create(module_name).unwrap();
                    std::io::Write::write_all(&mut file, module_bytes).unwrap();

                    let module = wasmi::Module::new(engine, module_bytes)?;
                    modules.push(module);
                }
                _ => {}
            }
        }

        println!("Found modules: {:?}", modules.len());

        Ok(Self {
            core_module: modules.remove(0),
        })
    }
}
