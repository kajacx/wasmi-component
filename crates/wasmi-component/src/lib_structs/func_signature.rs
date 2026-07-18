use wasmi_component_parser::ValueType;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FuncSignature {
    /// All params squished into a single tuple.
    pub params: ValueType,
    pub result: ValueType,
}

impl FuncSignature {
    pub fn new(params: ValueType, result: ValueType) -> Self {
        Self { params, result }
    }
}

impl std::fmt::Display for FuncSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        if let ValueType::Tuple(tuple) = &self.params {
            for (index, field) in tuple.iter().enumerate() {
                if index > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", field)?;
            }
        } else {
            write!(f, "{}", &self.params)?;
        }

        write!(f, ") -> {}", self.result)
    }
}
