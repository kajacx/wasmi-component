use wasmi_component_parser::Func;

use crate::type_helpers::{as_lower, canonical_name, as_lift, rust_snake_case};

pub fn params_full_lift(func: &Func) -> String {
    func.params
        .iter()
        .map(|(name, ty)| format!("{}: {}, ", rust_snake_case(name), as_lift(ty)))
        .collect()
}

pub fn params_full_lower(func: &Func) -> String {
    func.params
        .iter()
        .map(|(name, ty)| format!("{}: {}, ", rust_snake_case(name), as_lower(ty)))
        .collect()
}

pub fn param_types_canon(func: &Func) -> String {
    func.params
        .iter()
        .map(|(_name, ty)| format!("{}, ", canonical_name(ty)))
        .collect()
}

pub fn param_names_as_args(func: &Func) -> String {
    func.params
        .iter()
        .map(|(name, _ty)| format!("{}, ", rust_snake_case(name)))
        .collect()
}

pub fn params_indexes(func: &Func) -> String {
    (0..func.params.len())
        .map(|index| format!("params.{index}, "))
        .collect()
}
