use std::borrow::Cow;

use crate::ValueType;

#[derive(Debug, Clone, Default)]
pub struct Func {
    pub ident: FuncIdentifier,
    pub params: Vec<(String, ValueType)>,
    pub result: ValueType,
}

impl Func {
    pub fn new(
        module: String,
        name: String,
        params: Vec<(String, ValueType)>,
        result: ValueType,
    ) -> Self {
        Self {
            ident: FuncIdentifier::new(module, name),
            params,
            result,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuncIdentifier {
    /// Used for both imported and exported function, can be empty if there is no module.
    pub module: String, // TODO: Rc<str>?

    /// Name of the function. Will have the name after the # for exported functions.
    pub name: String,
}

impl FuncIdentifier {
    pub fn new(module: String, name: String) -> Self {
        Self { module, name }
    }

    pub fn imported_module_name(&self) -> &str {
        if self.module.is_empty() {
            "$root"
        } else {
            &self.module
        }
    }

    pub fn exported_name(&self) -> Cow<'_, str> {
        if self.module.is_empty() {
            self.name.as_str().into()
        } else {
            format!("{}#{}", self.module, self.name).into()
        }
    }
}

impl std::fmt::Display for FuncIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.module.is_empty() {
            write!(f, "(no module)#{}", self.name)
        } else {
            write!(f, "{}#{}", self.module, self.name)
        }
    }
}
