pub use wasmi;
pub use wasmi::Engine;

mod component;
mod instance;
mod linker;
mod store;

pub use component::*;
pub use instance::*;
pub use linker::*;
pub use store::*;
