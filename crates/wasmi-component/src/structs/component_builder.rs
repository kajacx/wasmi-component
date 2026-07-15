use std::collections::HashMap;

use anyhow::{Context, bail};
use wasmparser::{Parser, Payload};
use wit_component::decode;

use crate::FuncSignature;

#[derive(Debug, Clone)]
pub(crate) struct ComponentBuilder<'a> {
    modules: Vec<&'a [u8]>,
}

impl<'a> ComponentBuilder<'a> {
    pub fn new(bytes: &'a [u8]) -> anyhow::Result<Self> {
        let parser = Parser::new(0);

        let mut modules: Vec<&'a [u8]> = Vec::new();

        for payload in parser.parse_all(bytes) {
            match payload? {
                Payload::ModuleSection {
                    unchecked_range, ..
                } => {
                    modules.push(&bytes[unchecked_range]);
                }
                _ => {}
            }
        }

        let a = decode(bytes)?;
        a.resolve();

        Ok(Self { modules })
    }

    pub fn core_module(&self) -> anyhow::Result<&[u8]> {
        self.modules
            .get(0)
            .copied()
            .context("component doesn't have any core modules")
    }

    pub fn imported_funcs(&self) -> anyhow::Result<HashMap<String, FuncSignature>> {
        let mut output = HashMap::new();

        Ok(output)
    }

    pub fn exported_funcs(&self) -> anyhow::Result<HashMap<String, FuncSignature>> {
        let mut output = HashMap::new();

        Ok(output)
    }
}
