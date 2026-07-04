use wasmi::ValType;

mod primitive;
mod string;
mod tuple;

pub trait FlatArgs {
    fn arg_count() -> usize;

    fn arg_types() -> Vec<ValType>;

    fn byte_align() -> usize;

    fn byte_size() -> usize;
}
