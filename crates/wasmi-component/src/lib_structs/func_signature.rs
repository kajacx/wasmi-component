use wasmi_component_parser::ValueType;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FuncSignature {
    pub params: Vec<ValueType>,
    pub result: ValueType,
}

impl FuncSignature {
    pub fn new(params: Vec<ValueType>, result: ValueType) -> Self {
        Self { params, result }
    }

    /// Tries to unbox arguments if `params` is a tuple, otherwise it just makes a function of one argument.
    pub fn new_grouped(params: ValueType, result: ValueType) -> Self {
        let params = if let ValueType::Tuple(tuple) = params {
            tuple.clone()
        } else {
            vec![params]
        };

        Self { params, result }
    }
}

impl std::fmt::Display for FuncSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        for (index, field) in self.params.iter().enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", field)?;
        }

        write!(f, ") -> {}", self.result)
    }
}
