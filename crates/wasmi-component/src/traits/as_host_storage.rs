use crate::HostStorage;

pub trait AsHostStorage {
    fn as_host_storage(&self) -> &HostStorage;

    fn as_host_storage_mut(&mut self) -> &mut HostStorage;
}

impl AsHostStorage for HostStorage {
    fn as_host_storage(&self) -> &HostStorage {
        self
    }

    fn as_host_storage_mut(&mut self) -> &mut HostStorage {
        self
    }
}
