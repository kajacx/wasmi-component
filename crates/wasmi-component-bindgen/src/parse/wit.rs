use crate::parse::ParsedWorld;

pub struct ParsedWit {
    #[allow(unused)] // TODO: unused
    pub types: Vec<String>,
    pub worlds: Vec<ParsedWorld>,
}
