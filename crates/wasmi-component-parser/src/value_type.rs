#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
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

    /// If this type is a tuple of one element, returns that element, otherwise returns self.
    /// Works recursively.
    fn unbox_single_tuple(&self) -> &Self {
        let mut result = self;
        while let Self::Tuple(tuple) = result {
            if tuple.len() == 1 {
                result = &tuple[0];
            } else {
                break;
            }
        }
        result
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
                    return write!(f, "()");
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

impl Default for ValueType {
    fn default() -> Self {
        Self::unit()
    }
}

impl PartialEq for ValueType {
    fn eq(&self, other: &Self) -> bool {
        let a = self.unbox_single_tuple();
        let b = other.unbox_single_tuple();

        match (a, b) {
            (Self::S8, Self::S8) => true,
            (Self::S16, Self::S16) => true,
            (Self::S32, Self::S32) => true,
            (Self::S64, Self::S64) => true,

            (Self::U8, Self::U8) => true,
            (Self::U16, Self::U16) => true,
            (Self::U32, Self::U32) => true,
            (Self::U64, Self::U64) => true,

            (Self::F32, Self::F32) => true,
            (Self::F64, Self::F64) => true,

            (Self::Bool, Self::Bool) => true,
            (Self::Char, Self::Char) => true,

            (Self::String, Self::String) => true,

            (Self::Option(opt_a), Self::Option(opt_b)) => opt_a == opt_b,
            (Self::Result(ok_a, err_a), Self::Result(ok_b, err_b)) => {
                ok_a == ok_b && err_a == err_b
            }
            (Self::Tuple(tuple_a), Self::Tuple(tuple_b)) => tuple_a == tuple_b,
            (Self::List(ty_a), Self::List(ty_b)) => ty_a == ty_b,

            (Self::Record { fields: a, .. }, Self::Record { fields: b, .. }) => iters_eq(
                a.iter().map(|(_name, ty)| ty),
                b.iter().map(|(_name, ty)| ty),
            ),
            (Self::Variant { cases: a, .. }, Self::Variant { cases: b, .. }) => iters_eq(
                a.iter().map(|(_name, ty)| ty),
                b.iter().map(|(_name, ty)| ty),
            ),

            _ => false,
        }
    }
}

fn iters_eq<T: PartialEq<T>>(
    iter1: impl Iterator<Item = T>,
    iter2: impl Iterator<Item = T>,
) -> bool {
    iter1.eq(iter2)
}
