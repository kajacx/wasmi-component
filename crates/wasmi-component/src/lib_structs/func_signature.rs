use std::rc::Rc;

use wasmi_component_parser::ValueType;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FuncSignature {
    pub params: Rc<[ValueType]>,
    pub result: ValueType,
}

impl FuncSignature {
    pub fn new(params: Rc<[ValueType]>, result: ValueType) -> Self {
        Self { params, result }
    }

    pub fn from_vec(params: Vec<ValueType>, result: ValueType) -> Self {
        Self {
            params: Rc::from(params),
            result,
        }
    }

    /// Tries to unbox arguments if `params` is a tuple, otherwise it just makes a function of one argument.
    pub fn from_grouped(params: ValueType, result: ValueType) -> Self {
        let params = if let ValueType::Tuple(tuple) = params {
            tuple.clone()
        } else {
            Rc::from([params])
        };

        Self { params, result }
    }

    /// Returns params of this function as a single tuple
    pub fn params_as_tuple(&self) -> ValueType {
        ValueType::Tuple(self.params.clone())
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
