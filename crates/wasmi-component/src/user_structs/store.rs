use std::rc::Rc;

use anyhow::{Context, Result, ensure};
use wasmi::{AsContext, AsContextMut, Engine};

use crate::{
    Component,
    lib_structs::{ComponentBuilder, MemoryAccessPre},
};

pub struct Store<T> {
    store: wasmi::Store<StoreData<T>>,
}

pub struct StoreData<T> {
    data: T,

    pub(crate) components: Vec<Component>,
    memory_table: Vec<MemoryAccessPre>,

    instance_call_stack: Vec<usize>,
}

impl<T> Store<T> {
    pub fn new(engine: &Engine, data: T) -> Self {
        let store_data = StoreData {
            data,
            components: Vec::new(),
            memory_table: Vec::new(),
            instance_call_stack: Vec::new(),
        };

        Self {
            store: wasmi::Store::new(engine, store_data),
        }
    }

    pub fn engine(&self) -> &Engine {
        self.store.engine()
    }

    pub fn data(&self) -> &T {
        &self.store.data().data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.store.data_mut().data
    }

    pub fn new_component(&mut self, bytes: &[u8]) -> anyhow::Result<Component> {
        let builder = ComponentBuilder::new(bytes)?;

        let core_module = wasmi::Module::new(self.engine(), builder.core_module()?)?;
        let imported_funcs = Rc::new(builder.imported_funcs());
        let exported_funcs = Rc::new(builder.exported_funcs());

        let component = Component {
            index: self.store.data().components.len(),
            core_module,
            imported_funcs,
            exported_funcs,
        };

        self.store.data_mut().components.push(component.clone());
        Ok(component)
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

    pub fn current_call_instance(&self) -> Option<usize> {
        self.instance_call_stack.last().copied()
    }

    pub(crate) fn push_call_instance(&mut self, id: usize) -> usize {
        self.instance_call_stack.push(id);
        self.instance_call_stack.len()
    }

    pub(crate) fn pop_call_instance(&mut self, id: usize, depth: usize) -> Result<()> {
        ensure!(
            self.instance_call_stack.len() == depth,
            "Call stack depth {} does not equal expected depth of {}",
            self.instance_call_stack.len(),
            depth
        );

        let popped = self
            .instance_call_stack
            .pop()
            .context("call stack is empty")?;

        ensure!(
            popped == id,
            "Tried to pop instance id {id}, but found {popped} instead."
        );

        Ok(())
    }
}

impl<T> AsContext for Store<T> {
    type Data = StoreData<T>;

    fn as_context(&self) -> wasmi::StoreContext<'_, Self::Data> {
        self.store.as_context()
    }
}

impl<T> AsContextMut for Store<T> {
    fn as_context_mut(&mut self) -> wasmi::StoreContextMut<'_, Self::Data> {
        self.store.as_context_mut()
    }
}
