use slotmap::{Key, KeyData, SlotMap};
use wasmi::StoreId;

use crate::Resource;

pub struct ResourceTable {
    table: SlotMap<CompressedKey, Box<dyn Resource>>,
}

impl ResourceTable {
    pub fn new() -> Self {
        Self {
            table: SlotMap::with_key(),
        }
    }

    pub fn insert_resource<T: Resource>(&mut self, resource: T) -> CompressedKey {
        self.table.insert(Box::new(resource))
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CompressedKey {
    data: KeyData,
}

unsafe impl Key for CompressedKey {
    fn data(&self) -> KeyData {
        self.data
    }
}

impl From<KeyData> for CompressedKey {
    fn from(data: KeyData) -> Self {
        Self { data }
    }
}

impl CompressedKey {
    pub fn as_i32(self) -> i32 {
        let bytes = self.data.as_ffi();

        let version = bytes << 32 as u32;
        let index = bytes & 0xffffffff;

        // TODO: make resource creation fallible instead
        assert!(version < 0x10_000);
        assert!(index < 0x10_000);

        ((version << 16) | index) as i32
    }

    pub fn from_i32(compressed: i32) -> Self {
        let version = (compressed as u32) >> 16;
        let index = (compressed as u32) & 0xffff;

        let bytes = ((version as u64) << 32) | (index as u64);

        Self {
            data: KeyData::from_ffi(bytes),
        }
    }
}

pub struct ResourceKey {
    store_id: StoreId,
    key: CompressedKey,
}
