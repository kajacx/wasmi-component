use std::rc::Rc;

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

    Option(Rc<ValueType>),
    Result(Rc<ValueType>, Rc<ValueType>),
    Tuple(Rc<[ValueType]>),
    List(Rc<ValueType>),

    Record {
        name: Rc<str>,
        fields: Rc<[(Rc<str>, ValueType)]>,
    },
    Variant {
        name: Rc<str>,
        cases: Rc<[(Rc<str>, Option<ValueType>)]>,
    },
}

impl ValueType {
    pub fn new_option(inner: ValueType) -> Self {
        Self::Option(Rc::new(inner))
    }

    pub fn new_result(ok: ValueType, err: ValueType) -> Self {
        Self::Result(Rc::new(ok), Rc::new(err))
    }

    pub fn new_tuple(fields: impl IntoIterator<Item = ValueType>) -> Self {
        Self::Tuple(fields.into_iter().collect())
    }

    pub fn new_list(inner: ValueType) -> Self {
        Self::List(Rc::new(inner))
    }

    pub fn new_unit() -> Self {
        Self::new_tuple([])
    }

    /// Returns the INNER type if this is an option type, otherwise returns None.
    pub fn as_option(&self) -> Option<&ValueType> {
        match self {
            Self::Option(ty) => Some(ty),
            _ => None,
        }
    }

    /// Returns the (ok, err) inner types if this is a result type, otherwise returns None.
    pub fn as_result(&self) -> Option<(&ValueType, &ValueType)> {
        match self {
            Self::Result(ok, err) => Some((ok, err)),
            _ => None,
        }
    }

    /// Returns the inner types if this is a tuple type, otherwise returns None.
    pub fn as_tuple(&self) -> Option<&[ValueType]> {
        match self {
            Self::Tuple(types) => Some(types),
            _ => None,
        }
    }

    /// Returns the inner type if this is a list type, otherwise returns None.
    pub fn as_list(&self) -> Option<&ValueType> {
        match self {
            Self::List(ty) => Some(ty),
            _ => None,
        }
    }

    /// Returns the name and fields of a record type, or None if this is not a record type.
    pub fn as_record(&self) -> Option<(&str, &[(Rc<str>, ValueType)])> {
        match self {
            Self::Record { name, fields } => Some((name, fields)),
            _ => None,
        }
    }

    /// Returns the name and cases of a variant type, or None if this is not a variant type.
    pub fn as_variant(&self) -> Option<(&str, &[(Rc<str>, Option<ValueType>)])> {
        match self {
            Self::Variant { name, cases } => Some((name, cases)),
            _ => None,
        }
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

    /// Returns the inner type for a list
    pub fn list_type(&self) -> Option<&ValueType> {
        match self {
            Self::List(ty) => Some(ty),
            _ => None,
        }
    }

    pub fn arg_count(&self) -> usize {
        match self {
            Self::S8 => 1,
            Self::S16 => 1,
            Self::S32 => 1,
            Self::S64 => 1,

            Self::U8 => 1,
            Self::U16 => 1,
            Self::U32 => 1,
            Self::U64 => 1,

            Self::F32 => 1,
            Self::F64 => 1,

            Self::Bool => 1,
            Self::Char => 1,
            Self::String => 2,

            Self::Option(ty) => 1 + ty.arg_count(),
            Self::Result(ok, err) => 1 + std::cmp::max(ok.arg_count(), err.arg_count()),
            Self::Tuple(fields) => fields.iter().map(|ty| ty.arg_count()).sum(),
            Self::List(_) => 2,

            Self::Record { fields, .. } => fields.iter().map(|(_name, ty)| ty.arg_count()).sum(),
            Self::Variant { cases, .. } => {
                1 + cases
                    .iter()
                    .map(|(_name, ty)| ty.as_ref().map_or(0, |ty| ty.arg_count()))
                    .max()
                    .unwrap_or(0)
            }
        }
    }

    pub fn byte_align(&self) -> usize {
        match self {
            Self::S8 => 1,
            Self::S16 => 2,
            Self::S32 => 4,
            Self::S64 => 8,

            Self::U8 => 1,
            Self::U16 => 2,
            Self::U32 => 4,
            Self::U64 => 8,

            Self::F32 => 4,
            Self::F64 => 8,

            Self::Bool => 1,
            Self::Char => 4,
            Self::String => 4,

            Self::Option(ty) => ty.byte_align(),
            Self::Result(ok, err) => std::cmp::max(ok.byte_align(), err.byte_align()),
            Self::Tuple(fields) => fields.iter().map(|ty| ty.byte_align()).max().unwrap_or(1),
            Self::List(_) => 4,

            Self::Record { fields, .. } => fields
                .iter()
                .map(|(_name, ty)| ty.byte_align())
                .max()
                .unwrap_or(1),
            Self::Variant { cases, .. } => {
                // TODO: variant with more than 256 cases
                cases
                    .iter()
                    .map(|(_name, ty)| ty.as_ref().map_or(1, |ty| ty.byte_align()))
                    .max()
                    .unwrap_or(1)
            }
        }
    }

    pub fn byte_size(&self) -> usize {
        match self {
            Self::S8 => 1,
            Self::S16 => 2,
            Self::S32 => 4,
            Self::S64 => 8,

            Self::U8 => 1,
            Self::U16 => 2,
            Self::U32 => 4,
            Self::U64 => 8,

            Self::F32 => 4,
            Self::F64 => 8,

            Self::Bool => 1,
            Self::Char => 4,
            Self::String => 8,

            Self::Option(ty) => self.byte_align() + ty.byte_size(),
            Self::Result(ok, err) => self.byte_align() + ok.byte_size() + err.byte_size(),
            Self::Tuple(fields) => fields.iter().map(|ty| ty.byte_align()).max().unwrap_or(1),
            Self::List(_) => 8,

            Self::Record { fields, .. } => fields
                .iter()
                .map(|(_name, ty)| ty.byte_align())
                .max()
                .unwrap_or(1),
            Self::Variant { cases, .. } => {
                // TODO: variant with more than 256 cases
                cases
                    .iter()
                    .map(|(_name, ty)| ty.as_ref().map_or(1, |ty| ty.byte_align()))
                    .max()
                    .unwrap_or(1)
            }
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
        Self::new_unit()
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

            // TODO: compare names, but case is different (snake case, etc.)
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
