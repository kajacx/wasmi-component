use wasmi_component_parser::ValueType;

use crate::helpers::round_up;
use crate::lib_structs::{MemoryAccess, WasmValue};
use crate::{ComponentValue, ConvertResult, DynValue, Lower, dyn_lower};

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

    fn write_dyn_field(
        &mut self,
        ty: &ValueType,
        value: &DynValue,
        align: usize,
    ) -> ConvertResult<()>;

    fn write_enum_determinant(&mut self, case_count: usize, determinant: usize, align: usize) {
        match case_count {
            ..0x1_00 => self
                .write_record_field::<u8>(determinant as u8, align)
                .unwrap(),
            ..0x1_00_00 => self
                .write_record_field::<u16>(determinant as u16, align)
                .unwrap(),
            ..0x1_00_00_00_00 => self
                .write_record_field::<u32>(determinant as u32, align)
                .unwrap(),
            _ => unimplemented!("variant has more than 2^32 cases"),
        }
    }

    fn write_variant<T: ComponentValue, C: ComponentValue>(
        &mut self,
        case_count: usize,
        determinant: usize,
        value: impl Lower<C>,
    ) -> ConvertResult<()>;

    fn write_dyn_variant(
        &mut self,
        variant_ty: &ValueType,
        determinant: usize,
        value: Option<(&ValueType, &DynValue)>,
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

    fn write_dyn_field(
        &mut self,
        ty: &ValueType,
        value: &DynValue,
        _align: usize,
    ) -> ConvertResult<()> {
        dyn_lower(ty, value, self)
    }

    fn write_variant<T: ComponentValue, C: ComponentValue>(
        &mut self,
        _case_count: usize,
        determinant: usize,
        value: impl Lower<C>,
    ) -> ConvertResult<()> {
        let final_index = self.index + T::arg_count();

        self.write_u32(determinant as _);
        let result = value.lower(self)?;

        self.index = final_index;
        Ok(result)
    }

    fn write_dyn_variant(
        &mut self,
        variant_ty: &ValueType,
        determinant: usize,
        value: Option<(&ValueType, &DynValue)>,
    ) -> ConvertResult<()> {
        let final_index = self.index + variant_ty.arg_count();

        self.write_u32(determinant as _);
        if let Some((ty, value)) = value {
            dyn_lower(ty, value, self)?;
        }

        self.index = final_index;
        Ok(())
    }

    fn memory(&mut self) -> &mut impl MemoryAccess {
        self.memory_access.re_borrow()
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

    fn write_dyn_field(
        &mut self,
        ty: &ValueType,
        value: &DynValue,
        align: usize,
    ) -> ConvertResult<()> {
        dyn_lower(ty, value, self)?;
        self.index = round_up(self.index, align);
        Ok(())
    }

    fn write_variant<T: ComponentValue, C: ComponentValue>(
        &mut self,
        case_count: usize,
        determinant: usize,
        value: impl Lower<C>,
    ) -> ConvertResult<()> {
        let final_index = self.index + T::byte_size();

        self.write_enum_determinant(case_count, determinant, T::byte_align());
        value.lower(self)?;

        self.index = final_index;
        Ok(())
    }

    fn write_dyn_variant(
        &mut self,
        variant_ty: &ValueType,
        determinant: usize,
        value: Option<(&ValueType, &DynValue)>,
    ) -> ConvertResult<()> {
        let final_index = self.index + variant_ty.byte_size();

        let case_count = variant_ty.case_count().expect("type was checked earlier");
        self.write_enum_determinant(case_count, determinant, variant_ty.byte_align());
        if let Some((ty, value)) = value {
            dyn_lower(ty, value, self)?;
        }

        self.index = final_index;
        Ok(())
    }

    fn memory(&mut self) -> &mut impl MemoryAccess {
        self.memory_access.re_borrow()
    }
}
