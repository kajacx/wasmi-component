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

    Record(Vec<(String, ValueType)>),
    Variant(Vec<(String, Option<ValueType>)>),
}
