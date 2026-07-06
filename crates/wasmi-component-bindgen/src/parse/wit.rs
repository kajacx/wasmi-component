use crate::parse::ParsedWorld;

pub struct ParsedWit {
    pub types: Vec<String>,
    pub worlds: Vec<ParsedWorld>,
}
