use wasmi::{AsContext, AsContextMut, Engine};

use crate::{MemoryAccessPre, ResourceTable};

pub struct Store<T> {
    inner: wasmi::Store<StoreData<T>>,
}

pub struct StoreData<T> {
    data: T,
    memory_table: Vec<MemoryAccessPre>,
    #[allow(unused)]
    resource_table: ResourceTable,
}

impl<T> Store<T> {
    pub fn new(engine: &Engine, data: T) -> Self {
        let store_data = StoreData {
            data,
            memory_table: Vec::new(),
            resource_table: ResourceTable::new(),
        };

        Self {
            inner: wasmi::Store::new(engine, store_data),
        }
    }

    pub fn data(&self) -> &T {
        &self.inner.data().data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.inner.data_mut().data
    }
}

impl<T> StoreData<T> {
    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn next_memory_index(&self) -> usize {
        self.memory_table.len()
    }

    pub fn insert_memory(&mut self, index: usize, memory: MemoryAccessPre) {
        assert_eq!(index, self.memory_table.len(), "Incorrect memory index");
        self.memory_table.push(memory);
    }

    pub fn get_memory(&self, index: usize) -> &MemoryAccessPre {
        &self.memory_table[index]
    }
}

impl<T> AsContext for Store<T> {
    type Data = StoreData<T>;

    fn as_context(&self) -> wasmi::StoreContext<'_, Self::Data> {
        self.inner.as_context()
    }
}

impl<T> AsContextMut for Store<T> {
    fn as_context_mut(&mut self) -> wasmi::StoreContextMut<'_, Self::Data> {
        self.inner.as_context_mut()
    }
}
