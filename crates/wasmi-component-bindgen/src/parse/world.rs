use crate::parse::Func;

pub struct ParsedWorld {
    pub world_name: String,
    pub imports: Vec<Func>,
    pub exports: Vec<Func>,
}
