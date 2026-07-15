use crate::{ParsedWorld, ValueType};

pub struct ParsedWit {
    pub types: Vec<ValueType>,
    pub worlds: Vec<ParsedWorld>,
}
