#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValueType {
    S8,
    S16,
    S32,
    S64,

    U8,
    U16,
    U32,
    U64,

    F32,
    F64,

    Bool,
    Char,

    String,

    Option(Box<ValueType>),
    Result(Box<ValueType>, Box<ValueType>),
    Tuple(Vec<ValueType>),
    List(Box<ValueType>),

    Record {
        name: String,
        fields: Vec<(String, ValueType)>,
    },
    Variant {
        name: String,
        cases: Vec<(String, Option<ValueType>)>,
    },
}

impl ValueType {
    pub fn unit() -> Self {
        Self::Tuple(vec![])
    }

    pub fn is_unit(&self) -> bool {
        match self {
            Self::Tuple(tuple) => tuple.is_empty(),
            _ => false,
        }
    }
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::S8 => write!(f, "s8"),
            Self::S16 => write!(f, "s16"),
            Self::S32 => write!(f, "s32"),
            Self::S64 => write!(f, "s64"),

            Self::U8 => write!(f, "u8"),
            Self::U16 => write!(f, "u16"),
            Self::U32 => write!(f, "u32"),
            Self::U64 => write!(f, "u64"),

            Self::F32 => write!(f, "f32"),
            Self::F64 => write!(f, "f64"),

            Self::Bool => write!(f, "bool"),
            Self::Char => write!(f, "char"),

            Self::String => write!(f, "string"),

            Self::Option(ty) => write!(f, "option<{ty}>"),
            Self::Result(ok, err) => write!(f, "result<{ok}, {err}>"),
            Self::Tuple(tuple) => {
                if tuple.len() == 0 {
                    // Result unused type
                    return write!(f, "_");
                }
                write!(f, "tuple<")?;
                for (index, field) in tuple.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?
                    }
                    write!(f, "{field}")?
                }
                write!(f, ">")
            }
            Self::List(ty) => write!(f, "list<{ty}>"),

            Self::Record { name, .. } => {
                if name.is_empty() {
                    write!(f, "anonymous record")
                } else {
                    write!(f, "{name}")
                }
            }
            Self::Variant { name, .. } => {
                if name.is_empty() {
                    write!(f, "anonymous variant")
                } else {
                    write!(f, "{name}")
                }
            }
        }
    }
}
