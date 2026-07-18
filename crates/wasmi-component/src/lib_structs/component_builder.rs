use anyhow::{Context, bail};
use wasmi_component_parser::{ParsedWorld, ValueType};
use wasmparser::{Parser, Payload};
use wit_component::{DecodedWasm, decode};

use crate::lib_structs::{FuncSignature, FuncStorage};

#[derive(Debug, Clone)]
pub(crate) struct ComponentBuilder<'a> {
    modules: Vec<&'a [u8]>,
    world: ParsedWorld,
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

        let world = if let DecodedWasm::Component(resolve, world_id) = decode(bytes)? {
            let parser = wasmi_component_parser::Parser::new(resolve);

            // TODO: parser should be fallible instead of unwrapping
            parser.parse_world(
                parser
                    .resolve
                    .worlds
                    .get(world_id)
                    .as_ref()
                    .context("get world by id")?,
            )
        } else {
            bail!("component bytes did not decode into a component")
        };

        Ok(Self { modules, world })
    }

    pub fn core_module(&self) -> anyhow::Result<&[u8]> {
        self.modules
            .get(0)
            .copied()
            .context("component doesn't have any core modules")
    }

    pub fn imported_funcs(&self) -> FuncStorage {
        let mut storage = FuncStorage::new();
        for func in &self.world.imports {
            println!("processing WORLD IMPORT {}", func.ident);
            storage.insert(
                func.ident.clone(),
                FuncSignature::new(
                    ValueType::Tuple(func.params.iter().map(|(_name, ty)| ty.clone()).collect()),
                    func.result.clone(),
                ),
            );
        }
        storage
    }

    pub fn exported_funcs(&self) -> FuncStorage {
        let mut storage = FuncStorage::new();
        for func in &self.world.exports {
            storage.insert(
                func.ident.clone(),
                FuncSignature::new(
                    ValueType::Tuple(func.params.iter().map(|(_name, ty)| ty.clone()).collect()),
                    func.result.clone(),
                ),
            );
        }
        storage
    }
}
