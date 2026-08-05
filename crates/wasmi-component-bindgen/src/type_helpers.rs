use heck::{ToSnakeCase, ToUpperCamelCase};
use wasmi_component_parser::ValueType;

static KEYWORDS: [&'static str; 38] = [
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while",
];

pub fn rust_snake_case(name: impl AsRef<str>) -> String {
    let mut name = name.as_ref().to_snake_case();
    if KEYWORDS.contains(&name.as_str()) {
        name.push('_');
    }
    name
}

/// Full canonical name, for example `list<string>` -> `Vec<String>`.
/// This will implement `ComponentValue`. Examples:
/// - `s32` -> `i32`
/// - `string` -> `String`
/// - `list<i32>` -> `Bytes<i32>` TODO: actually do this with bytemuck
/// - `list<string>` -> `Vec<String>`
/// - `option<list<u8>>` -> `Option<Bytes<u8>>`
/// - `custom-type` -> `CustomType`
pub fn canonical_name(ty: &ValueType) -> String {
    match ty {
        ValueType::S8 => "i8".into(),
        ValueType::S16 => "i16".into(),
        ValueType::S32 => "i32".into(),
        ValueType::S64 => "i64".into(),

        ValueType::U8 => "u8".into(),
        ValueType::U16 => "u16".into(),
        ValueType::U32 => "u32".into(),
        ValueType::U64 => "u64".into(),

        ValueType::F32 => "f32".into(),
        ValueType::F64 => "f64".into(),

        ValueType::Bool => "bool".into(),
        ValueType::Char => "char".into(),
        ValueType::String => "String".into(),

        ValueType::Option(ty) => format!("Option<{}>", canonical_name(ty)),
        ValueType::Result(ok, err) => {
            format!("Result<{}, {}>", canonical_name(ok), canonical_name(err))
        }
        ValueType::Tuple(tuple) => {
            let mut result = String::from("(");
            for ty in tuple.iter() {
                result.push_str(&canonical_name(ty));
                result.push_str(", ");
            }
            result.push(')');
            result
        }
        ValueType::List(ty) => format!("Vec<{}>", canonical_name(ty)),

        ValueType::Record { name, .. } => name.to_upper_camel_case(),
        ValueType::Variant { name, .. } => name.to_upper_camel_case(),
    }
}

/// `Lift` is exported function's result or imported function's arguments. Examples:
/// - `s32` -> `i32`
/// - `string` -> `&str`
/// - `list<i32>` -> `&[i32]` TODO: actually do this with bytemuck
/// - `list<string>` -> `ListAccessor<String>`
/// - `option<list<u8>>` -> `Option<&[u8]>`
/// - `custom-type` -> `CustomTypeBorrowed`
pub fn as_lift(ty: &ValueType) -> String {
    match ty {
        ValueType::S8 => "i8".into(),
        ValueType::S16 => "i16".into(),
        ValueType::S32 => "i32".into(),
        ValueType::S64 => "i64".into(),

        ValueType::U8 => "u8".into(),
        ValueType::U16 => "u16".into(),
        ValueType::U32 => "u32".into(),
        ValueType::U64 => "u64".into(),

        ValueType::F32 => "f32".into(),
        ValueType::F64 => "f64".into(),

        ValueType::Bool => "bool".into(),
        ValueType::Char => "char".into(),
        ValueType::String => "&str".into(),

        ValueType::Option(ty) => format!("Option<{}>", as_lift(ty)),
        ValueType::Result(ok, err) => {
            format!("Result<{}, {}>", as_lift(ok), as_lift(err))
        }
        ValueType::Tuple(tuple) => {
            let mut result = String::from("(");
            for ty in tuple.iter() {
                result.push_str(&as_lift(ty));
                result.push_str(", ");
            }
            result.push(')');
            result.into()
        }
        ValueType::List(ty) => format!("ListAccessor<'_, {}>", canonical_name(ty)).into(),

        ValueType::Record { name, .. } => format!("{}Borrowed<'_>", name.to_upper_camel_case()),
        ValueType::Variant { name, .. } => format!("{}Borrowed<'_>", name.to_upper_camel_case()),
    }
}

/// `Lower` is currently used only in exported function arguments. Examples:
/// - `s32` -> `i32`
/// - `string` -> `&str`
/// - `list<i32>` -> `impl Lower<Vec<i32>>`
/// - `list<string>` -> `impl Lower<Vec<String>>`
/// - `option<list<u8>>` -> `impl Lower<Option<Vec<u8>>>`
/// - `custom-type` -> `&CustomType`
pub fn as_lower(ty: &ValueType) -> String {
    match ty {
        ValueType::S8 => "i8".into(),
        ValueType::S16 => "i16".into(),
        ValueType::S32 => "i32".into(),
        ValueType::S64 => "i64".into(),

        ValueType::U8 => "u8".into(),
        ValueType::U16 => "u16".into(),
        ValueType::U32 => "u32".into(),
        ValueType::U64 => "u64".into(),

        ValueType::F32 => "f32".into(),
        ValueType::F64 => "f64".into(),

        ValueType::Bool => "bool".into(),
        ValueType::Char => "char".into(),
        ValueType::String => "&str".into(),

        ValueType::Option(_)
        | ValueType::Result(_, _)
        | ValueType::Tuple(_)
        | ValueType::List(_) => {
            if ty.is_unit() {
                "()".into()
            } else {
                format!("impl Lower<{}>", canonical_name(ty))
            }
        }

        ValueType::Record { name, .. } => format!("&{}", name.to_upper_camel_case()),
        ValueType::Variant { name, .. } => format!("&{}", name.to_upper_camel_case()),
    }
}
