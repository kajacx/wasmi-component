mod bindgen;
mod impl_pls;

pub mod resources;

pub use bindgen::add_root_to_linker as add_wasi_p2_to_linker;
