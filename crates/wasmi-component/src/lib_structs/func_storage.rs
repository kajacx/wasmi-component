use std::fmt::Write;

use anyhow::{Context, bail};
use wasmi_component_parser::FuncIdentifier;

use crate::{ConvertError, lib_structs::FuncSignature};

#[derive(Debug, Clone, Default)]
pub struct FuncStorage {
    pub data: Vec<(FuncIdentifier, FuncSignature)>,
}

impl FuncStorage {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn insert(&mut self, ident: FuncIdentifier, signature: FuncSignature) {
        self.data.retain(|(id, _)| id != &ident);
        self.data.push((ident, signature));
    }

    pub fn get(&self, ident: &FuncIdentifier) -> Option<&FuncSignature> {
        self.data
            .iter()
            .find(|(id, _)| id == ident)
            .map(|(_, signature)| signature)
    }

    pub fn verify_import(
        &self,
        ident: &FuncIdentifier,
        signature: &FuncSignature,
    ) -> anyhow::Result<()> {
        let host_signature = self.get(ident).ok_or_else(|| {
            ConvertError::new("dynamic imported function not found in the component")
                .with_additional(format!(
                    "imported function \"{}\" is not present, defined functions are: {:?}",
                    ident,
                    self.existing_fn_names()
                ))
        })?;

        if host_signature != signature {
            bail!(
                "imported function \"{}\" has invalid signature: component expected {}, but host has {} instead",
                ident,
                signature,
                host_signature
            );
        }

        Ok(())
    }

    pub fn verify_export(
        &self,
        ident: &FuncIdentifier,
        signature: &FuncSignature,
    ) -> anyhow::Result<()> {
        let existing_guest = self.get(ident).with_context(|| {
            format!(
                "exported function \"{}\" is not present, existing functions are: {:?}",
                ident,
                self.data
                    .iter()
                    .map(|(ident, _)| ident.to_string())
                    .collect::<Vec<_>>()
            )
        })?;

        if existing_guest != signature {
            bail!(
                "exported function \"{}\" has invalid signature: host expected {}, but component has {} instead",
                ident,
                signature,
                existing_guest
            );
        }

        Ok(())
    }

    pub fn existing_fn_names(&self) -> String {
        let mut output = String::from("[");
        for (index, func) in self.data.iter().enumerate() {
            if index > 0 {
                output.push_str(", ");
            }
            write!(output, "\"{}\"", func.0).unwrap();
        }
        output.push(']');
        output
    }
}
