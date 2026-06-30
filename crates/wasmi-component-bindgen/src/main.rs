use clap::Parser as CliParser;
use std::path::{Path, PathBuf};
use wit_parser::UnresolvedPackageGroup;

#[derive(CliParser)]
#[command(version, about = "Experimental zero-copy component bindgen for wasmi")]
struct Args {
    /// Path to the input WIT file.
    wit: PathBuf,

    /// Name of the world to generate bindings for.
    #[arg(short, long)]
    world: Option<String>,
}

fn main() {
    let args = Args::parse();

    let content = std::fs::read_to_string(&args.wit).unwrap();
    let path = Path::new(&args.wit);
    let group = UnresolvedPackageGroup::parse(path, &content).unwrap();
}
