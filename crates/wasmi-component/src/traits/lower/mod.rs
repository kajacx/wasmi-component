use wasmi::ValType;

mod primitive;
mod string;
mod tuple;

pub trait Lower {
    fn params_count() -> usize;

    fn imported_result() -> Vec<ValType>;
}
