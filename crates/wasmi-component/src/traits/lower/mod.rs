use wasmi::ValType;

use crate::FlatArgs;

mod primitive;
mod string;
mod tuple;

pub trait Lower: FlatArgs {}
