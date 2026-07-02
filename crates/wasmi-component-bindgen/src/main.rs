use std::path::{Path, PathBuf};

use wit_parser::UnresolvedPackageGroup;

mod generator;
mod parse;

use generator::*;
use parse::*;

#[derive(clap::Parser)]
#[command(version, about = "Experimental zero-copy component bindgen for wasmi")]
struct Args {
    /// Path to the input WIT file.
    wit: PathBuf,
}

fn main() {
    let args = <Args as clap::Parser>::parse();

    let content = std::fs::read_to_string(&args.wit).unwrap();
    let path = Path::new(&args.wit);
    let group = UnresolvedPackageGroup::parse(path, &content).unwrap();

    let parser = Parser::new(group.main);
    let worlds = parser.parse_wit();

    let mut output = String::new();
    generate_wit(&worlds, &mut output);

    print!("{output}");
}
