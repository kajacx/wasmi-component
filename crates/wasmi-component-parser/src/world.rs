use crate::Func;

#[derive(Debug, Clone, Default)]
pub struct ParsedWorld {
    pub name: String,
    pub imports: Vec<Func>,
    pub exports: Vec<Func>,
}

impl ParsedWorld {
    pub fn new(name: String, imports: Vec<Func>, exports: Vec<Func>) -> Self {
        Self {
            name,
            imports,
            exports,
        }
    }
}
