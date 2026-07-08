use std::path::{Path, PathBuf};

use wit_parser::{Resolve, UnresolvedPackageGroup};

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

    let mut resolve = Resolve::new();
    resolve.push_group(group.clone()).unwrap();

    let parser = Parser::new(resolve);
    let wit = parser.parse_wit();

    let generator = Generator::new();
    let output = generator.generate_wit(wit);

    print!("{output}");
}
