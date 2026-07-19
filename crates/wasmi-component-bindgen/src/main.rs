use std::path::{Path, PathBuf};

use wasmi_component_parser::Parser;
use wit_parser::{Resolve, UnresolvedPackageGroup};

mod func_helpers;
mod generator;
mod type_helpers;

use generator::*;

#[derive(clap::Parser)]
#[command(version, about = "Experimental zero-copy component bindgen for wasmi")]
struct Args {
    /// Path to the input WIT file.
    wit: PathBuf,

    /// Implement ComponentValue manually instead of using a macro.
    #[arg(short, long, default_value_t = false)]
    manual_impl: bool,
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

    let generator = Generator::new(args.manual_impl);
    let output = generator.generate_wit(wit);

    print!("{output}");
}
