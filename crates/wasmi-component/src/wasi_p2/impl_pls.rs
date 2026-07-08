use std::io::Write;

use crate::{
    Borrow, CompValue, HostResult, ListAccessor, LowerVal, Own, StoreData, wasi_p2::resources::*,
};

#[allow(unused)]
impl<D> super::bindgen::RootImports for StoreData<D> {
    fn method_pollable_block(&mut self, self_: Borrow<PollableResource>) -> HostResult<()> {
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
        self_: Borrow<InputStreamResource>,
        len: u64,
    ) -> HostResult<impl LowerVal<Result<Vec<u8>, StreamError>> + 'static> {
        todo!();
        Ok(Result::<Vec<u8>, StreamError>::Ok(vec![]))
    }

    fn method_input_stream_subscribe(
        &mut self,
        self_: Borrow<InputStreamResource>,
    ) -> HostResult<Own<PollableResource>> {
        todo!();
        Ok(Own::<PollableResource>::new(0))
    }

    fn method_output_stream_check_write(
        &mut self,
        self_: Borrow<OutputStreamResource>,
    ) -> HostResult<Result<u64, StreamError>> {
        println!("Calling method_output_stream_check_write");

        Ok(Result::<u64, StreamError>::Ok(4 * 1024 * 1024))
    }

    fn method_output_stream_write(
        &mut self,
        self_: Borrow<OutputStreamResource>,
        contents: ListAccessor<u8>,
    ) -> HostResult<Result<(), StreamError>> {
        println!("Calling method_output_stream_write");

        std::io::stdout()
            .write_all(contents.as_u8_slice())
            .expect("write to stdout");

        Ok(Ok(()))
    }

    fn method_output_stream_blocking_flush(
        &mut self,
        self_: Borrow<OutputStreamResource>,
    ) -> HostResult<Result<(), StreamError>> {
        println!("Calling method_output_stream_blocking_flush");

        Ok(Ok(()))
    }

    fn method_output_stream_subscribe(
        &mut self,
        self_: Borrow<OutputStreamResource>,
    ) -> HostResult<Own<PollableResource>> {
        println!("Calling method_output_stream_subscribe");

        Ok(Own::new(0))
    }

    fn resource_drop_input_stream(&mut self, index: i32) -> HostResult<()> {
        todo!()
    }

    fn resource_drop_output_stream(&mut self, index: i32) -> HostResult<()> {
        todo!()
    }

    fn get_environment(&mut self) -> HostResult<impl LowerVal<Vec<(String, String)>> + 'static> {
        todo!();
        Ok(Vec::<(String, String)>::new())
    }

    fn exit(&mut self, status: <Result<(), ()> as CompValue>::Borrowed<'_>) -> HostResult<()> {
        todo!()
    }

    fn get_stdin(&mut self) -> HostResult<Own<InputStreamResource>> {
        todo!();
        Ok(Own::new(0))
    }

    fn get_stdout(&mut self) -> HostResult<Own<OutputStreamResource>> {
        println!("Calling get_stdout");

        Ok(Own::new(0))
    }

    fn get_stderr(&mut self) -> HostResult<Own<OutputStreamResource>> {
        println!("Calling get_stderr");

        Ok(Own::new(0))
    }

    fn resource_drop_terminal_input(&mut self, index: i32) -> HostResult<()> {
        todo!()
    }

    fn resource_drop_terminal_output(&mut self, index: i32) -> HostResult<()> {
        todo!()
    }

    fn get_terminal_stdin(&mut self) -> HostResult<Option<Own<TerminalInputResource>>> {
        Ok(None)
    }

    fn get_terminal_stdout(&mut self) -> HostResult<Option<Own<TerminalOutputResource>>> {
        Ok(None)
    }

    fn get_terminal_stderr(&mut self) -> HostResult<Option<Own<TerminalOutputResource>>> {
        Ok(None)
    }
}
