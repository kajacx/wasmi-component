use crate::anyhow::Result;
#[allow(unused)]
use crate::wasi_p2::{add_wasi_p2_to_linker, resources::*};
#[allow(unused)]
use crate::wasmi::{AsContext, AsContextMut};
#[allow(unused)]
use crate::{
    Borrow, Component, ComponentValue, HostResult, Linker, ListAccessor, LowerVal, Own, StoreData,
    TypedFunc,
};

#[allow(unused)]
pub trait RootImports {
    fn method_pollable_block(&mut self, self_: Borrow<PollableResource>) -> HostResult<()>;

    fn resource_drop_pollable(&mut self, index: i32) -> HostResult<()>;

    fn resource_drop_error(&mut self, index: i32) -> HostResult<()>;

    type MethodInputStreamBlockingReadReturn: LowerVal<Result<Vec<u8>, StreamError>> + 'static;

    fn method_input_stream_blocking_read(
        &mut self,
        self_: Borrow<InputStreamResource>,
        len: u64,
    ) -> HostResult<Self::MethodInputStreamBlockingReadReturn>;

    fn method_input_stream_subscribe(
        &mut self,
        self_: Borrow<InputStreamResource>,
    ) -> HostResult<Own<PollableResource>>;

    fn method_output_stream_check_write(
        &mut self,
        self_: Borrow<OutputStreamResource>,
    ) -> HostResult<Result<u64, StreamError>>;

    fn method_output_stream_write(
        &mut self,
        self_: Borrow<OutputStreamResource>,
        contents: ListAccessor<u8>,
    ) -> HostResult<Result<(), StreamError>>;

    fn method_output_stream_blocking_flush(
        &mut self,
        self_: Borrow<OutputStreamResource>,
    ) -> HostResult<Result<(), StreamError>>;

    fn method_output_stream_subscribe(
        &mut self,
        self_: Borrow<OutputStreamResource>,
    ) -> HostResult<Own<PollableResource>>;

    fn resource_drop_input_stream(&mut self, index: i32) -> HostResult<()>;

    fn resource_drop_output_stream(&mut self, index: i32) -> HostResult<()>;

    type GetEnvironmentReturn: LowerVal<Vec<(String, String)>> + 'static;

    fn get_environment(&mut self) -> HostResult<Self::GetEnvironmentReturn>;

    fn exit(&mut self, status: Result<(), ()>) -> HostResult<()>;

    fn get_stdin(&mut self) -> HostResult<Own<InputStreamResource>>;

    fn get_stdout(&mut self) -> HostResult<Own<OutputStreamResource>>;

    fn get_stderr(&mut self) -> HostResult<Own<OutputStreamResource>>;

    fn resource_drop_terminal_input(&mut self, index: i32) -> HostResult<()>;

    fn resource_drop_terminal_output(&mut self, index: i32) -> HostResult<()>;

    fn get_terminal_stdin(&mut self) -> HostResult<Option<Own<TerminalInputResource>>>;

    fn get_terminal_stdout(&mut self) -> HostResult<Option<Own<TerminalOutputResource>>>;

    fn get_terminal_stderr(&mut self) -> HostResult<Option<Own<TerminalOutputResource>>>;
}

#[allow(unused)]
pub struct RootExports {}

#[allow(unused)]
pub fn add_root_to_linker<T>(linker: &mut Linker<T>) -> Result<()> {
    linker.func_direct::<(Borrow<PollableResource>,), (), _>(
        "wasi:io/poll@0.2.0",
        "[method]pollable.block",
        |host_data, params| host_data.method_pollable_block(params.0),
    )?;

    linker.func_direct::<(i32,), (), _>(
        "wasi:io/poll@0.2.0",
        "[resource-drop]pollable",
        |host_data, params| host_data.resource_drop_pollable(params.0),
    )?;

    linker.func_direct::<(i32,), (), _>(
        "wasi:io/error@0.2.0",
        "[resource-drop]error",
        |host_data, params| host_data.resource_drop_error(params.0),
    )?;

    linker.func_direct::<(Borrow<InputStreamResource>, u64), Result<Vec<u8>, StreamError>, _>(
        "wasi:io/streams@0.2.0",
        "[method]input-stream.blocking-read",
        |host_data, params| host_data.method_input_stream_blocking_read(params.0, params.1),
    )?;

    linker.func_direct::<(Borrow<InputStreamResource>,), Own<PollableResource>, _>(
        "wasi:io/streams@0.2.0",
        "[method]input-stream.subscribe",
        |host_data, params| host_data.method_input_stream_subscribe(params.0),
    )?;

    linker.func_direct::<(Borrow<OutputStreamResource>,), Result<u64, StreamError>, _>(
        "wasi:io/streams@0.2.0",
        "[method]output-stream.check-write",
        |host_data, params| host_data.method_output_stream_check_write(params.0),
    )?;

    linker.func_direct::<(Borrow<OutputStreamResource>, Vec<u8>), Result<(), StreamError>, _>(
        "wasi:io/streams@0.2.0",
        "[method]output-stream.write",
        |host_data, params| host_data.method_output_stream_write(params.0, params.1),
    )?;

    linker.func_direct::<(Borrow<OutputStreamResource>,), Result<(), StreamError>, _>(
        "wasi:io/streams@0.2.0",
        "[method]output-stream.blocking-flush",
        |host_data, params| host_data.method_output_stream_blocking_flush(params.0),
    )?;

    linker.func_direct::<(Borrow<OutputStreamResource>,), Own<PollableResource>, _>(
        "wasi:io/streams@0.2.0",
        "[method]output-stream.subscribe",
        |host_data, params| host_data.method_output_stream_subscribe(params.0),
    )?;

    linker.func_direct::<(i32,), (), _>(
        "wasi:io/streams@0.2.0",
        "[resource-drop]input-stream",
        |host_data, params| host_data.resource_drop_input_stream(params.0),
    )?;

    linker.func_direct::<(i32,), (), _>(
        "wasi:io/streams@0.2.0",
        "[resource-drop]output-stream",
        |host_data, params| host_data.resource_drop_output_stream(params.0),
    )?;

    linker.func_direct::<(), Vec<(String, String)>, _>(
        "wasi:cli/environment@0.2.0",
        "get-environment",
        |host_data, params| host_data.get_environment(),
    )?;

    linker.func_direct::<(Result<(), ()>,), (), _>(
        "wasi:cli/exit@0.2.0",
        "exit",
        |host_data, params| host_data.exit(params.0),
    )?;

    linker.func_direct::<(), Own<InputStreamResource>, _>(
        "wasi:cli/stdin@0.2.0",
        "get-stdin",
        |host_data, params| host_data.get_stdin(),
    )?;

    linker.func_direct::<(), Own<OutputStreamResource>, _>(
        "wasi:cli/stdout@0.2.0",
        "get-stdout",
        |host_data, params| host_data.get_stdout(),
    )?;

    linker.func_direct::<(), Own<OutputStreamResource>, _>(
        "wasi:cli/stderr@0.2.0",
        "get-stderr",
        |host_data, params| host_data.get_stderr(),
    )?;

    linker.func_direct::<(i32,), (), _>(
        "wasi:cli/terminal-input@0.2.0",
        "[resource-drop]terminal-input",
        |host_data, params| host_data.resource_drop_terminal_input(params.0),
    )?;

    linker.func_direct::<(i32,), (), _>(
        "wasi:cli/terminal-output@0.2.0",
        "[resource-drop]terminal-output",
        |host_data, params| host_data.resource_drop_terminal_output(params.0),
    )?;

    linker.func_direct::<(), Option<Own<TerminalInputResource>>, _>(
        "wasi:cli/terminal-stdin@0.2.0",
        "get-terminal-stdin",
        |host_data, params| host_data.get_terminal_stdin(),
    )?;

    linker.func_direct::<(), Option<Own<TerminalOutputResource>>, _>(
        "wasi:cli/terminal-stdout@0.2.0",
        "get-terminal-stdout",
        |host_data, params| host_data.get_terminal_stdout(),
    )?;

    linker.func_direct::<(), Option<Own<TerminalOutputResource>>, _>(
        "wasi:cli/terminal-stderr@0.2.0",
        "get-terminal-stderr",
        |host_data, params| host_data.get_terminal_stderr(),
    )?;

    Ok(())
}

#[allow(unused)]
pub fn instantiate_root_world<T>(
    mut ctx: impl AsContextMut<Data = StoreData<T>>,
    linker: &Linker<T>,
    component: &Component,
) -> Result<RootExports> {
    let instance = linker.instantiate(ctx.as_context_mut(), &component)?;

    Ok(RootExports {})
}
