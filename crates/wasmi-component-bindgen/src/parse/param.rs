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

pub enum LowerArg {
    LowerVal,
    Specific(String),
}

impl LowerArg {
    pub fn specific(&self) -> Option<&str> {
        match self {
            Self::Specific(val) => Some(val),
            Self::LowerVal => None,
        }
    }
}
