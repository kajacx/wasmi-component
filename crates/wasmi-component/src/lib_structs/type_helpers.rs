use crate::{ValueType, helpers::variant_types};

pub fn wasm_args(ty: &ValueType) -> Vec<wasmi::ValType> {
    match ty {
        ValueType::S8 => vec![wasmi::ValType::I32],
        ValueType::S16 => vec![wasmi::ValType::I32],
        ValueType::S32 => vec![wasmi::ValType::I32],
        ValueType::S64 => vec![wasmi::ValType::I64],

        ValueType::U8 => vec![wasmi::ValType::I32],
        ValueType::U16 => vec![wasmi::ValType::I32],
        ValueType::U32 => vec![wasmi::ValType::I32],
        ValueType::U64 => vec![wasmi::ValType::I64],

        ValueType::F32 => vec![wasmi::ValType::F32],
        ValueType::F64 => vec![wasmi::ValType::F64],

        ValueType::Bool => vec![wasmi::ValType::I32],
        ValueType::Char => vec![wasmi::ValType::I32],
        ValueType::String => vec![wasmi::ValType::I32, wasmi::ValType::I32],

        ValueType::Option(ty) => variant_types([wasm_args(ty)]),
        ValueType::Result(ok, err) => variant_types([wasm_args(ok), wasm_args(err)]),
        ValueType::Tuple(fields) => fields.iter().flat_map(wasm_args).collect(),
        ValueType::List(_) => vec![wasmi::ValType::I32, wasmi::ValType::I32],

        ValueType::Record { fields, .. } => fields
            .iter()
            .flat_map(|(_name, ty)| wasm_args(ty))
            .collect(),
        ValueType::Variant { cases, .. } => variant_types(
            cases
                .iter()
                .filter_map(|(_name, ty)| ty.as_ref().map(wasm_args)),
        ),
        ValueType::Enum { .. } => vec![wasmi::ValType::I32],
    }
}

pub fn enum_determinant_size(case_count: usize) -> usize {
    match case_count {
        ..0x1_00 => 1,
        ..0x1_00_00 => 2,
        ..0x1_00_00_00_00 => 4,
        _ => unimplemented!("enum has more than 2^32 cases"),
    }
}
