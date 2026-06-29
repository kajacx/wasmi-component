use crate::*;

#[derive(Debug)]
pub struct Component {}

impl Component {
    pub fn new(_engine: &Engine, _bytes: &[u8]) -> Result<Self, wasmi::Error> {
        Ok(Self {})
    }
}
