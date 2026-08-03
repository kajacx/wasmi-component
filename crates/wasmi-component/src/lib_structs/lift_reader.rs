use wasmi_component_parser::ValueType;

use crate::helpers::round_up;
use crate::lib_structs::WasmValue;
use crate::pointers::FatPtr;
use crate::{ComponentValue, ConvertResult, DynValue, dyn_lift};

pub trait LiftReader<'mem>: Sized {
    fn read_u8(&mut self) -> u8;
    fn read_u16(&mut self) -> u16;
    fn read_u32(&mut self) -> u32;
    fn read_u64(&mut self) -> u64;

    fn read_f32(&mut self) -> f32;
    fn read_f64(&mut self) -> f64;

    fn read_fat_ptr(&mut self, size: usize) -> FatPtr {
        let (start, count) = <(u32, u32)>::lift(self).expect("reading u32 cannot fail");
        FatPtr::new(start as usize, count as usize, size)
    }

    fn read_record_field<T: ComponentValue>(
        &mut self,
        align: usize,
    ) -> ConvertResult<T::Borrowed<'mem>>;

    fn read_dyn_field(&mut self, ty: &ValueType, align: usize) -> ConvertResult<DynValue>;

    fn read_variant<T: ComponentValue>(
        &mut self,
        cases: impl FnOnce(&mut Self, usize) -> ConvertResult<T::Borrowed<'mem>>,
    ) -> ConvertResult<T::Borrowed<'mem>>;

    fn read_dyn_variant(
        &mut self,
        ty: &ValueType,
        cases: impl FnOnce(&mut Self, usize) -> ConvertResult<DynValue>,
    ) -> ConvertResult<DynValue>;

    fn memory(&self) -> &'mem [u8];
}

pub(crate) struct LiftArgsReader<'mem, 'a> {
    memory: &'mem [u8],
    args: &'a [WasmValue],
    index: usize,
}

impl<'mem, 'a> LiftArgsReader<'mem, 'a> {
    pub fn new(memory: &'mem [u8], args: &'a [WasmValue]) -> Self {
        Self {
            memory,
            args,
            index: 0,
        }
    }

    fn next_arg(&mut self) -> &WasmValue {
        let result = &self.args[self.index];
        self.index += 1;
        result
    }
}

impl<'mem, 'a> LiftReader<'mem> for LiftArgsReader<'mem, 'a> {
    fn read_u8(&mut self) -> u8 {
        self.next_arg().i32().expect("arg types were checked") as _
    }

    fn read_u16(&mut self) -> u16 {
        self.next_arg().i32().expect("arg types were checked") as _
    }

    fn read_u32(&mut self) -> u32 {
        self.next_arg().i32().expect("arg types were checked") as _
    }

    fn read_u64(&mut self) -> u64 {
        self.next_arg().i64().expect("arg types were checked") as _
    }

    fn read_f32(&mut self) -> f32 {
        self.next_arg().f32().expect("arg types were checked")
    }

    fn read_f64(&mut self) -> f64 {
        self.next_arg().f64().expect("arg types were checked")
    }

    fn read_record_field<T: ComponentValue>(
        &mut self,
        _align: usize,
    ) -> ConvertResult<T::Borrowed<'mem>> {
        T::lift(self)
    }

    fn read_dyn_field(&mut self, ty: &ValueType, _align: usize) -> ConvertResult<DynValue> {
        dyn_lift(ty, self)
    }

    fn read_variant<T: ComponentValue>(
        &mut self,
        cases: impl FnOnce(&mut Self, usize) -> ConvertResult<T::Borrowed<'mem>>,
    ) -> ConvertResult<T::Borrowed<'mem>> {
        let final_index = self.index + T::arg_count();
        let determinant = self.read_u8(); // TODO: variants with more than 256 cases

        let result = cases(self, determinant as usize)?;

        self.index = final_index;
        Ok(result)
    }

    fn read_dyn_variant(
        &mut self,
        ty: &ValueType,
        cases: impl FnOnce(&mut Self, usize) -> ConvertResult<DynValue>,
    ) -> ConvertResult<DynValue> {
        let final_index = self.index + ty.arg_count();
        let determinant = self.read_u8(); // TODO: variants with more than 256 cases

        let result = cases(self, determinant as usize)?;

        self.index = final_index;
        Ok(result)
    }

    fn memory(&self) -> &'mem [u8] {
        self.memory
    }
}

pub(crate) struct LiftBytesReader<'mem, 'a> {
    memory: &'mem [u8],
    bytes: &'a [u8],
    index: usize,
}

impl<'mem, 'a> LiftBytesReader<'mem, 'a> {
    pub fn new(memory: &'mem [u8], bytes: &'a [u8]) -> Self {
        Self {
            memory,
            bytes,
            index: 0,
        }
    }
}

impl<'mem, 'a> LiftReader<'mem> for LiftBytesReader<'mem, 'a> {
    fn read_u8(&mut self) -> u8 {
        let result = self.bytes[self.index];
        self.index += 1;
        result
    }

    fn read_u16(&mut self) -> u16 {
        let result =
            u16::from_le_bytes(self.bytes[self.index..(self.index + 2)].try_into().unwrap());
        self.index += 2;
        result as _
    }

    fn read_u32(&mut self) -> u32 {
        let result =
            u32::from_le_bytes(self.bytes[self.index..(self.index + 4)].try_into().unwrap());
        self.index += 4;
        result
    }

    fn read_u64(&mut self) -> u64 {
        let result =
            u64::from_le_bytes(self.bytes[self.index..(self.index + 8)].try_into().unwrap());
        self.index += 8;
        result
    }

    fn read_f32(&mut self) -> f32 {
        f32::from_bits(self.read_u32())
    }

    fn read_f64(&mut self) -> f64 {
        f64::from_bits(self.read_u64())
    }

    fn read_record_field<T: ComponentValue>(
        &mut self,
        align: usize,
    ) -> ConvertResult<T::Borrowed<'mem>> {
        let result = T::lift(self)?;
        self.index = round_up(self.index, align);
        Ok(result)
    }

    fn read_dyn_field(&mut self, ty: &ValueType, align: usize) -> ConvertResult<DynValue> {
        let result = dyn_lift(ty, self)?;
        self.index = round_up(self.index, align);
        Ok(result)
    }

    fn read_variant<T: ComponentValue>(
        &mut self,
        cases: impl FnOnce(&mut Self, usize) -> ConvertResult<T::Borrowed<'mem>>,
    ) -> ConvertResult<T::Borrowed<'mem>> {
        let final_index = self.index + T::byte_size();
        let determinant = self
            .read_record_field::<u8>(T::byte_align()) // TODO: variants with more than 256 cases
            .expect("reading u8 cannot fail");

        let result = cases(self, determinant as usize)?;

        self.index = final_index;
        Ok(result)
    }

    fn read_dyn_variant(
        &mut self,
        ty: &ValueType,
        cases: impl FnOnce(&mut Self, usize) -> ConvertResult<DynValue>,
    ) -> ConvertResult<DynValue> {
        let final_index = self.index + ty.byte_align();
        let determinant = self
            .read_record_field::<u8>(ty.byte_align()) // TODO: variants with more than 256 cases
            .expect("reading u8 cannot fail");

        let result = cases(self, determinant as usize)?;

        self.index = final_index;
        Ok(result)
    }

    fn memory(&self) -> &'mem [u8] {
        &self.memory
    }
}
