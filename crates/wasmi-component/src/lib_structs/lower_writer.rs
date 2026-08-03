use crate::helpers::round_up;
use crate::lib_structs::{MemoryAccess, WasmValue};
use crate::{ComponentValue, ConvertResult, Lower};

pub trait LowerWriter {
    fn write_u8(&mut self, value: u8);
    fn write_u16(&mut self, value: u16);
    fn write_u32(&mut self, value: u32);
    fn write_u64(&mut self, value: u64);

    fn write_f32(&mut self, value: f32);
    fn write_f64(&mut self, value: f64);

    fn write_record_field<T: ComponentValue>(
        &mut self,
        value: impl Lower<T>,
        align: usize,
    ) -> ConvertResult<()>;

    fn write_variant<T: ComponentValue, C: ComponentValue>(
        &mut self,
        determinant: usize,
        value: impl Lower<C>,
    ) -> ConvertResult<()>;

    fn memory(&mut self) -> &mut impl MemoryAccess;
}

pub(crate) struct LowerArgsWriter<'a, M: MemoryAccess> {
    memory_access: M,
    args: &'a mut [WasmValue],
    index: usize,
}

impl<'a, M: MemoryAccess> LowerArgsWriter<'a, M> {
    pub fn new(memory_access: M, args: &'a mut [WasmValue]) -> Self {
        Self {
            memory_access,
            args,
            index: 0,
        }
    }

    fn next_arg(&mut self) -> &mut WasmValue {
        let result = &mut self.args[self.index];
        self.index += 1;
        result
    }
}

impl<'a, M: MemoryAccess> LowerWriter for LowerArgsWriter<'a, M> {
    fn write_u8(&mut self, value: u8) {
        *self.next_arg() = WasmValue::I32(value as _)
    }

    fn write_u16(&mut self, value: u16) {
        *self.next_arg() = WasmValue::I32(value as _)
    }

    fn write_u32(&mut self, value: u32) {
        *self.next_arg() = WasmValue::I32(value as _)
    }

    fn write_u64(&mut self, value: u64) {
        *self.next_arg() = WasmValue::I64(value as _)
    }

    fn write_f32(&mut self, value: f32) {
        *self.next_arg() = WasmValue::F32(value)
    }

    fn write_f64(&mut self, value: f64) {
        *self.next_arg() = WasmValue::F64(value)
    }

    fn write_record_field<T: ComponentValue>(
        &mut self,
        value: impl Lower<T>,
        _align: usize,
    ) -> ConvertResult<()> {
        value.lower(self)
    }

    fn write_variant<T: ComponentValue, C: ComponentValue>(
        &mut self,
        determinant: usize,
        value: impl Lower<C>,
    ) -> ConvertResult<()> {
        let final_index = self.index + T::arg_count();

        self.write_u8(determinant as _); // TODO: variants with more than 256 cases
        let result = value.lower(self)?;

        self.index = final_index;
        Ok(result)
    }

    fn memory(&mut self) -> &mut impl MemoryAccess {
        &mut self.memory_access
    }
}

pub(crate) struct LowerBytesWriter<M: MemoryAccess> {
    memory_access: M,
    index: usize,
}

impl<M: MemoryAccess> LowerBytesWriter<M> {
    pub fn new(memory_access: M, index: usize) -> Self {
        Self {
            memory_access,
            index,
        }
    }
}

impl<M: MemoryAccess> LowerWriter for LowerBytesWriter<M> {
    fn write_u8(&mut self, value: u8) {
        self.memory_access
            .slice(self.index..(self.index + 1))
            .expect("TODO:")
            .copy_from_slice(&value.to_le_bytes());
        self.index += 1;
    }

    fn write_u16(&mut self, value: u16) {
        self.memory_access
            .slice(self.index..(self.index + 2))
            .expect("TODO:")
            .copy_from_slice(&value.to_le_bytes());
        self.index += 2;
    }

    fn write_u32(&mut self, value: u32) {
        self.memory_access
            .slice(self.index..(self.index + 4))
            .expect("TODO:")
            .copy_from_slice(&value.to_le_bytes());
        self.index += 4;
    }

    fn write_u64(&mut self, value: u64) {
        self.memory_access
            .slice(self.index..(self.index + 8))
            .expect("TODO:")
            .copy_from_slice(&value.to_le_bytes());
        self.index += 8;
    }

    fn write_f32(&mut self, value: f32) {
        self.memory_access
            .slice(self.index..(self.index + 4))
            .expect("TODO:")
            .copy_from_slice(&value.to_le_bytes());
        self.index += 4;
    }

    fn write_f64(&mut self, value: f64) {
        self.memory_access
            .slice(self.index..(self.index + 8))
            .expect("TODO:")
            .copy_from_slice(&value.to_le_bytes());
        self.index += 8;
    }

    fn write_record_field<T: ComponentValue>(
        &mut self,
        value: impl Lower<T>,
        align: usize,
    ) -> ConvertResult<()> {
        value.lower(self)?;
        self.index = round_up(self.index, align);
        Ok(())
    }

    fn write_variant<T: ComponentValue, C: ComponentValue>(
        &mut self,
        determinant: usize,
        value: impl Lower<C>,
    ) -> ConvertResult<()> {
        let final_index = self.index + T::byte_size();

        // TODO: variants with more than 256 cases
        self.write_record_field(determinant as u8, T::byte_align())?;
        value.lower(self)?;

        self.index = final_index;
        Ok(())
    }

    fn memory(&mut self) -> &mut impl MemoryAccess {
        &mut self.memory_access
    }
}
