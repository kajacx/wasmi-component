use anyhow::{Context, bail};
use wasmi_component_parser::{FuncIdentifier, ParsedWorld, ValueType};
use wasmparser::{Parser, Payload};
use wit_component::{DecodedWasm, decode};

use crate::FuncSignature;

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

    pub fn imported_funcs(&self) -> anyhow::Result<FuncStorage> {
        let mut storage = FuncStorage::new();
        for func in &self.world.imports {
            storage.insert(
                func.ident.clone(),
                FuncSignature::new(
                    ValueType::Tuple(func.params.iter().map(|(_name, ty)| ty.clone()).collect()),
                    func.result.clone(),
                ),
            )?;
        }
        Ok(storage)
    }

    pub fn exported_funcs(&self) -> anyhow::Result<FuncStorage> {
        let mut storage = FuncStorage::new();
        for func in &self.world.exports {
            storage.insert(
                func.ident.clone(),
                FuncSignature::new(
                    ValueType::Tuple(func.params.iter().map(|(_name, ty)| ty.clone()).collect()),
                    func.result.clone(),
                ),
            )?;
        }
        Ok(storage)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FuncStorage {
    data: Vec<(FuncIdentifier, FuncSignature)>,
}

impl FuncStorage {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn insert(&mut self, ident: FuncIdentifier, signature: FuncSignature) -> anyhow::Result<()> {
        if self.get(&ident).is_some() {
            bail!("function \"{}\" cannot be re-inserted into storage", ident);
        }

        self.data.push((ident, signature));

        Ok(())
    }

    fn get(&self, ident: &FuncIdentifier) -> Option<&FuncSignature> {
        self.data
            .iter()
            .find(|(id, _)| id == ident)
            .map(|(_, signature)| signature)
    }

    pub fn verify(&self, ident: &FuncIdentifier, signature: &FuncSignature) -> anyhow::Result<()> {
        let existing = self.get(ident).with_context(|| {
            format!(
                "requested function \"{}\" is not present, existing functions are: {:?}",
                ident,
                self.data
                    .iter()
                    .map(|(ident, _)| ident.to_string())
                    .collect::<Vec<_>>()
            )
        })?;

        if existing != signature {
            bail!(
                "function \"{}\" has invalid signature: host expected {}, but component has {} instead",
                ident,
                signature,
                existing
            );
        }

        Ok(())
    }
}
