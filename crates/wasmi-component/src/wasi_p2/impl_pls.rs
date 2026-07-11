use std::io::Write;

use wasmi::ValType;

use crate::{ComponentValue, ConvertResult, HostResult, ListAccessor, LowerVal, ValueType, View};

#[allow(unused)]
impl<T> super::bindgen::RootImports for T {
    fn method_pollable_block(&mut self, self_: i32) -> HostResult<()> {
        todo!()
    }

    fn resource_drop_pollable(&mut self, index: i32) -> HostResult<()> {
        todo!()
    }

    fn resource_drop_error(&mut self, index: i32) -> HostResult<()> {
        todo!()
    }

    fn method_input_stream_blocking_read(
        &mut self,
        self_: i32,
        len: u64,
    ) -> HostResult<Result<Vec<u8>, StreamError>> {
        todo!();
    }

    fn method_input_stream_subscribe(&mut self, self_: i32) -> HostResult<i32> {
        todo!();
        Ok(0)
    }

    fn method_output_stream_check_write(
        &mut self,
        self_: i32,
    ) -> HostResult<Result<u64, StreamError>> {
        println!("Calling method_output_stream_check_write");

        Ok(Result::<u64, StreamError>::Ok(4 * 1024 * 1024))
    }

    fn method_output_stream_write(
        &mut self,
        self_: i32,
        contents: ListAccessor<u8>,
    ) -> HostResult<Result<(), StreamError>> {
        println!("Calling method_output_stream_write");

        if self_ == 0 {
            std::io::stdout()
                .write_all(contents.as_u8_slice())
                .expect("write to stdout");
        } else {
            std::io::stderr()
                .write_all(contents.as_u8_slice())
                .expect("write to stderr");
        }

        Ok(Ok(()))
    }

    fn method_output_stream_blocking_flush(
        &mut self,
        self_: i32,
    ) -> HostResult<Result<(), StreamError>> {
        println!("Calling method_output_stream_blocking_flush");

        Ok(Ok(()))
    }

    fn method_output_stream_subscribe(&mut self, self_: i32) -> HostResult<i32> {
        println!("Calling method_output_stream_subscribe");

        Ok(0)
    }

    fn resource_drop_input_stream(&mut self, index: i32) -> HostResult<()> {
        todo!()
    }

    fn resource_drop_output_stream(&mut self, index: i32) -> HostResult<()> {
        todo!()
    }

    fn get_environment(&mut self) -> HostResult<Vec<(String, String)>> {
        todo!();
    }

    fn exit(&mut self, status: <Result<(), ()> as ComponentValue>::Borrowed<'_>) -> HostResult<()> {
        todo!()
    }

    fn get_stdin(&mut self) -> HostResult<i32> {
        todo!();
        Ok(0)
    }

    fn get_stdout(&mut self) -> HostResult<i32> {
        println!("Calling get_stdout");

        Ok(0)
    }

    fn get_stderr(&mut self) -> HostResult<i32> {
        println!("Calling get_stderr");

        Ok(1)
    }

    fn resource_drop_terminal_input(&mut self, index: i32) -> HostResult<()> {
        todo!()
    }

    fn resource_drop_terminal_output(&mut self, index: i32) -> HostResult<()> {
        todo!()
    }

    fn get_terminal_stdin(&mut self) -> HostResult<Option<i32>> {
        Ok(None)
    }

    fn get_terminal_stdout(&mut self) -> HostResult<Option<i32>> {
        Ok(None)
    }

    fn get_terminal_stderr(&mut self) -> HostResult<Option<i32>> {
        Ok(None)
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum StreamError {
    LastOperationFailed(i32),
    Closed,
}

impl ComponentValue for StreamError {
    type Borrowed<'a> = Self;

    fn value_type() -> ValueType {
        ValueType::Variant(vec![
            ("LastOperationFailed".to_string(), Some(ValueType::S32)),
            ("Closed".to_string(), None),
        ])
    }

    fn arg_count() -> usize {
        1 + i32::arg_count()
    }

    fn arg_types() -> Vec<ValType> {
        let mut args = vec![ValType::I32];
        args.extend(i32::arg_types());
        args
    }

    fn lift_args<'a>(_vals: &[wasmi::Val], _memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        todo!()
    }

    fn byte_align() -> usize {
        i32::byte_align()
    }

    fn byte_size() -> usize {
        8
    }

    fn lift_bytes<'a>(_bytes: &[u8], _memory: &'a [u8]) -> ConvertResult<Self::Borrowed<'a>> {
        todo!()
    }
}

impl View<Self> for StreamError {
    fn lift_owned(&self) -> ConvertResult<Self> {
        Ok(self.clone())
    }

    fn lift_to(&self, target: &mut Self) -> ConvertResult<()> {
        *target = self.clone();
        Ok(())
    }
}

#[allow(unused)]
impl LowerVal<StreamError> for StreamError {
    fn lower_args(
        &self,
        args: &mut [wasmi::Val],
        memory: &mut impl crate::MemoryAccess,
    ) -> ConvertResult<()> {
        todo!()
    }

    fn lower_bytes(
        &self,
        range: std::ops::Range<usize>,
        memory: &mut impl crate::MemoryAccess,
    ) -> ConvertResult<()> {
        todo!()
    }
}
