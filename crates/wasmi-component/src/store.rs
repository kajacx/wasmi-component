use crate::*;

#[derive(Debug)]
pub struct Store<T> {
    pub store: wasmi::Store<T>,
}

impl<T> Store<T> {
    pub fn new(engine: &Engine, data: T) -> Self {
        let store = wasmi::Store::new(engine, data);
        Self { store }
    }
}

impl<T> wasmi::AsContext for Store<T> {
    type Data = T;

    fn as_context(&self) -> wasmi::StoreContext<'_, Self::Data> {
        self.store.as_context()
    }
}

impl<T> wasmi::AsContextMut for Store<T> {
    fn as_context_mut(&mut self) -> wasmi::StoreContextMut<'_, Self::Data> {
        self.store.as_context_mut()
    }
}
