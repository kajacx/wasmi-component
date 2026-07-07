use std::io::Write;

use crate::{Borrow, CompValue, IntoOwned, LowerVal, Own, StoreData, wasi_p2::resources::*};

#[allow(unused)]
impl<D> super::bindgen::RootImports for StoreData<D> {
    fn method_pollable_block(
        &mut self,
        self_: <Borrow<PollableResource> as CompValue>::Borrowed<'_>,
    ) -> crate::HostResult<()> {
        todo!()
    }

    fn resource_drop_pollable(&mut self, index: i32) -> crate::HostResult<()> {
        todo!()
    }

    fn resource_drop_error(&mut self, index: i32) -> crate::HostResult<()> {
        todo!()
    }

    fn method_input_stream_blocking_read(
        &mut self,
        self_: <Borrow<InputStreamResource> as CompValue>::Borrowed<'_>,
        len: u64,
    ) -> crate::HostResult<impl LowerVal<Result<Vec<u8>, StreamError>> + 'static> {
        todo!();
        Ok(Result::<Vec<u8>, StreamError>::Ok(vec![]))
    }

    fn method_input_stream_subscribe(
        &mut self,
        self_: <Borrow<InputStreamResource> as CompValue>::Borrowed<'_>,
    ) -> crate::HostResult<impl LowerVal<Own<PollableResource>> + 'static> {
        todo!();
        Ok(Own::<PollableResource>::new(0))
    }

    fn method_output_stream_check_write(
        &mut self,
        self_: <Borrow<OutputStreamResource> as CompValue>::Borrowed<'_>,
    ) -> crate::HostResult<impl LowerVal<Result<u64, StreamError>> + 'static> {
        println!("Calling method_output_stream_check_write");

        Ok(Result::<u64, StreamError>::Ok(4 * 1024 * 1024))
    }

    fn method_output_stream_write(
        &mut self,
        self_: <Borrow<OutputStreamResource> as CompValue>::Borrowed<'_>,
        contents: <Vec<u8> as CompValue>::Borrowed<'_>,
    ) -> crate::HostResult<impl LowerVal<Result<(), StreamError>> + 'static> {
        println!("Calling method_output_stream_write");

        let bytes = contents.into_owned(); // TODO: would be cool to not need this
        std::io::stdout()
            .write_all(&bytes)
            .expect("write to stdout");

        Ok(Result::<(), StreamError>::Ok(()))
    }

    fn method_output_stream_blocking_flush(
        &mut self,
        self_: <Borrow<OutputStreamResource> as CompValue>::Borrowed<'_>,
    ) -> crate::HostResult<impl LowerVal<Result<(), StreamError>> + 'static> {
        println!("Calling method_output_stream_blocking_flush");

        Ok(Result::<(), StreamError>::Ok(()))
    }

    fn method_output_stream_subscribe(
        &mut self,
        self_: <Borrow<OutputStreamResource> as CompValue>::Borrowed<'_>,
    ) -> crate::HostResult<impl LowerVal<Own<PollableResource>> + 'static> {
        println!("Calling method_output_stream_subscribe");

        Ok(Own::<PollableResource>::new(0))
    }

    fn resource_drop_input_stream(&mut self, index: i32) -> crate::HostResult<()> {
        todo!()
    }

    fn resource_drop_output_stream(&mut self, index: i32) -> crate::HostResult<()> {
        todo!()
    }

    fn get_environment(
        &mut self,
    ) -> crate::HostResult<impl LowerVal<Vec<(String, String)>> + 'static> {
        todo!();
        Ok(Vec::<(String, String)>::new())
    }

    fn exit(
        &mut self,
        status: <Result<(), ()> as CompValue>::Borrowed<'_>,
    ) -> crate::HostResult<()> {
        todo!()
    }

    fn get_stdin(
        &mut self,
    ) -> crate::HostResult<impl LowerVal<Own<InputStreamResource>> + 'static> {
        todo!();
        Ok(Own::<InputStreamResource>::new(0))
    }

    fn get_stdout(
        &mut self,
    ) -> crate::HostResult<impl LowerVal<Own<OutputStreamResource>> + 'static> {
        println!("Calling get_stdout");

        Ok(Own::<OutputStreamResource>::new(0))
    }

    fn get_stderr(
        &mut self,
    ) -> crate::HostResult<impl LowerVal<Own<OutputStreamResource>> + 'static> {
        println!("Calling get_stderr");

        Ok(Own::<OutputStreamResource>::new(0))
    }

    fn resource_drop_terminal_input(&mut self, index: i32) -> crate::HostResult<()> {
        todo!()
    }

    fn resource_drop_terminal_output(&mut self, index: i32) -> crate::HostResult<()> {
        todo!()
    }

    fn get_terminal_stdin(
        &mut self,
    ) -> crate::HostResult<impl LowerVal<Option<Own<TerminalInputResource>>> + 'static> {
        Ok(Option::<Own<TerminalInputResource>>::None)
    }

    fn get_terminal_stdout(
        &mut self,
    ) -> crate::HostResult<impl LowerVal<Option<Own<TerminalOutputResource>>> + 'static> {
        Ok(Option::<Own<TerminalOutputResource>>::None)
    }

    fn get_terminal_stderr(
        &mut self,
    ) -> crate::HostResult<impl LowerVal<Option<Own<TerminalOutputResource>>> + 'static> {
        Ok(Option::<Own<TerminalOutputResource>>::None)
    }
}
