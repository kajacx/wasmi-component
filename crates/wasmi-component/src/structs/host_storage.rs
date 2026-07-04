use crate::MemoryAccessPre;

#[derive(Clone, Debug, Default)]
pub struct HostStorage {
    memory_table: Vec<MemoryAccessPre>,
}

impl HostStorage {
    pub fn new() -> Self {
        Self {
            memory_table: Vec::new(),
        }
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
