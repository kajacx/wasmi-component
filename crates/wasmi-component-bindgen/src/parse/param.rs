// use std::fmt::{Display, write};

pub struct Param {
    pub name: String,
    pub ty: ParamType,
}

pub struct ParamType {
    pub canon: String,
    pub lower: LowerArg,
    pub lift: String,
}

impl ParamType {
    pub fn from_simple(name: &str) -> Self {
        Self {
            canon: name.to_string(),
            lower: LowerArg::Specific(name.to_string()),
            lift: name.to_string(),
        }
    }
}

pub enum LowerArg {
    LowerValue,
    Specific(String),
}

impl LowerArg {
    pub fn specific(&self) -> Option<&str> {
        match self {
            Self::Specific(val) => Some(val),
            Self::LowerValue => None,
        }
    }
}
