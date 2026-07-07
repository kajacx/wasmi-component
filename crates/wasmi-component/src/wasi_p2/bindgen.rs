use crate::anyhow::{Context, Result};
#[allow(unused)]
use crate::wasi_p2::{add_wasi_p2_to_linker, resources::*};
#[allow(unused)]
use crate::wasmi::{AsContext, AsContextMut, Caller, FuncType, Linker, ValType};
#[allow(unused)]
use crate::{
    Borrow, CompValue, Component, HostResult, LowerVal, MemoryAccessPre, Own, StoreData, TypedFunc,
    anyhow_result_to_wasmi,
};

#[allow(unused)]
pub trait RootImports {
    fn method_pollable_block(
        &mut self,
        self_: <Borrow<PollableResource> as CompValue>::Borrowed<'_>,
    ) -> HostResult<()>;

    fn resource_drop_pollable(&mut self, index: i32) -> HostResult<()>;

    fn resource_drop_error(&mut self, index: i32) -> HostResult<()>;

    fn method_input_stream_blocking_read(
        &mut self,
        self_: <Borrow<InputStreamResource> as CompValue>::Borrowed<'_>,
        len: u64,
    ) -> HostResult<impl LowerVal<Result<Vec<u8>, StreamError>> + 'static>;

    fn method_input_stream_subscribe(
        &mut self,
        self_: <Borrow<InputStreamResource> as CompValue>::Borrowed<'_>,
    ) -> HostResult<impl LowerVal<Own<PollableResource>> + 'static>;

    fn method_output_stream_check_write(
        &mut self,
        self_: <Borrow<OutputStreamResource> as CompValue>::Borrowed<'_>,
    ) -> HostResult<impl LowerVal<Result<u64, StreamError>> + 'static>;

    fn method_output_stream_write(
        &mut self,
        self_: <Borrow<OutputStreamResource> as CompValue>::Borrowed<'_>,
        contents: <Vec<u8> as CompValue>::Borrowed<'_>,
    ) -> HostResult<impl LowerVal<Result<(), StreamError>> + 'static>;

    fn method_output_stream_blocking_flush(
        &mut self,
        self_: <Borrow<OutputStreamResource> as CompValue>::Borrowed<'_>,
    ) -> HostResult<impl LowerVal<Result<(), StreamError>> + 'static>;

    fn method_output_stream_subscribe(
        &mut self,
        self_: <Borrow<OutputStreamResource> as CompValue>::Borrowed<'_>,
    ) -> HostResult<impl LowerVal<Own<PollableResource>> + 'static>;

    fn resource_drop_input_stream(&mut self, index: i32) -> HostResult<()>;

    fn resource_drop_output_stream(&mut self, index: i32) -> HostResult<()>;

    fn get_environment(&mut self) -> HostResult<impl LowerVal<Vec<(String, String)>> + 'static>;

    fn exit(&mut self, status: <Result<(), ()> as CompValue>::Borrowed<'_>) -> HostResult<()>;

    fn get_stdin(&mut self) -> HostResult<impl LowerVal<Own<InputStreamResource>> + 'static>;

    fn get_stdout(&mut self) -> HostResult<impl LowerVal<Own<OutputStreamResource>> + 'static>;

    fn get_stderr(&mut self) -> HostResult<impl LowerVal<Own<OutputStreamResource>> + 'static>;

    fn resource_drop_terminal_input(&mut self, index: i32) -> HostResult<()>;

    fn resource_drop_terminal_output(&mut self, index: i32) -> HostResult<()>;

    fn get_terminal_stdin(
        &mut self,
    ) -> HostResult<impl LowerVal<Option<Own<TerminalInputResource>>> + 'static>;

    fn get_terminal_stdout(
        &mut self,
    ) -> HostResult<impl LowerVal<Option<Own<TerminalOutputResource>>> + 'static>;

    fn get_terminal_stderr(
        &mut self,
    ) -> HostResult<impl LowerVal<Option<Own<TerminalOutputResource>>> + 'static>;
}

#[allow(unused)]
pub struct RootExports {}

#[allow(unused)]
pub fn add_root_to_linker<D>(
    mut ctx: impl AsContextMut<Data = StoreData<D>>,
    linker: &mut Linker<StoreData<D>>,
    memory_index: usize,
) -> Result<()> {
    let mut params_ty = <(Borrow<PollableResource>,)>::arg_types();
    let mut result_ty = <()>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:io/poll@0.2.0#[method]pollable.block");
    linker.func_new(
        "wasi:io/poll@0.2.0",
        "[method]pollable.block",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<(Borrow<PollableResource>,)>::lift_args(
                params_slice,
                bytes,
            ))?;
            let res = store_data.method_pollable_block(args.0)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <()>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(i32,)>::arg_types();
    let mut result_ty = <()>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:io/poll@0.2.0#[resource-drop]pollable");
    linker.func_new(
        "wasi:io/poll@0.2.0",
        "[resource-drop]pollable",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<(i32,)>::lift_args(params_slice, bytes))?;
            let res = store_data.resource_drop_pollable(args.0)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <()>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(i32,)>::arg_types();
    let mut result_ty = <()>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:io/error@0.2.0#[resource-drop]error");
    linker.func_new(
        "wasi:io/error@0.2.0",
        "[resource-drop]error",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<(i32,)>::lift_args(params_slice, bytes))?;
            let res = store_data.resource_drop_error(args.0)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <()>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(Borrow<InputStreamResource>, u64)>::arg_types();
    let mut result_ty = <Result<Vec<u8>, StreamError>>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:io/streams@0.2.0#[method]input-stream.blocking-read");
    linker.func_new(
        "wasi:io/streams@0.2.0",
        "[method]input-stream.blocking-read",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<(Borrow<InputStreamResource>, u64)>::lift_args(
                params_slice,
                bytes,
            ))?;
            let res = store_data.method_input_stream_blocking_read(args.0, args.1)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <Result<Vec<u8>, StreamError>>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(Borrow<InputStreamResource>,)>::arg_types();
    let mut result_ty = <Own<PollableResource>>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:io/streams@0.2.0#[method]input-stream.subscribe");
    linker.func_new(
        "wasi:io/streams@0.2.0",
        "[method]input-stream.subscribe",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<(Borrow<InputStreamResource>,)>::lift_args(
                params_slice,
                bytes,
            ))?;
            let res = store_data.method_input_stream_subscribe(args.0)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <Own<PollableResource>>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(Borrow<OutputStreamResource>,)>::arg_types();
    let mut result_ty = <Result<u64, StreamError>>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:io/streams@0.2.0#[method]output-stream.check-write");
    linker.func_new(
        "wasi:io/streams@0.2.0",
        "[method]output-stream.check-write",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<(Borrow<OutputStreamResource>,)>::lift_args(
                params_slice,
                bytes,
            ))?;
            let res = store_data.method_output_stream_check_write(args.0)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <Result<u64, StreamError>>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(Borrow<OutputStreamResource>, Vec<u8>)>::arg_types();
    let mut result_ty = <Result<(), StreamError>>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:io/streams@0.2.0#[method]output-stream.write");
    linker.func_new(
        "wasi:io/streams@0.2.0",
        "[method]output-stream.write",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(
                <(Borrow<OutputStreamResource>, Vec<u8>)>::lift_args(params_slice, bytes),
            )?;
            let res = store_data.method_output_stream_write(args.0, args.1)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <Result<(), StreamError>>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(Borrow<OutputStreamResource>,)>::arg_types();
    let mut result_ty = <Result<(), StreamError>>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:io/streams@0.2.0#[method]output-stream.blocking-flush");
    linker.func_new(
        "wasi:io/streams@0.2.0",
        "[method]output-stream.blocking-flush",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<(Borrow<OutputStreamResource>,)>::lift_args(
                params_slice,
                bytes,
            ))?;
            let res = store_data.method_output_stream_blocking_flush(args.0)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <Result<(), StreamError>>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(Borrow<OutputStreamResource>,)>::arg_types();
    let mut result_ty = <Own<PollableResource>>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:io/streams@0.2.0#[method]output-stream.subscribe");
    linker.func_new(
        "wasi:io/streams@0.2.0",
        "[method]output-stream.subscribe",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<(Borrow<OutputStreamResource>,)>::lift_args(
                params_slice,
                bytes,
            ))?;
            let res = store_data.method_output_stream_subscribe(args.0)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <Own<PollableResource>>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(i32,)>::arg_types();
    let mut result_ty = <()>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:io/streams@0.2.0#[resource-drop]input-stream");
    linker.func_new(
        "wasi:io/streams@0.2.0",
        "[resource-drop]input-stream",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<(i32,)>::lift_args(params_slice, bytes))?;
            let res = store_data.resource_drop_input_stream(args.0)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <()>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(i32,)>::arg_types();
    let mut result_ty = <()>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:io/streams@0.2.0#[resource-drop]output-stream");
    linker.func_new(
        "wasi:io/streams@0.2.0",
        "[resource-drop]output-stream",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<(i32,)>::lift_args(params_slice, bytes))?;
            let res = store_data.resource_drop_output_stream(args.0)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <()>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <()>::arg_types();
    let mut result_ty = <Vec<(String, String)>>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:cli/environment@0.2.0#get-environment");
    linker.func_new(
        "wasi:cli/environment@0.2.0",
        "get-environment",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<()>::lift_args(params_slice, bytes))?;
            let res = store_data.get_environment()?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <Vec<(String, String)>>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(Result<(), ()>,)>::arg_types();
    let mut result_ty = <()>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:cli/exit@0.2.0#exit");
    linker.func_new(
        "wasi:cli/exit@0.2.0",
        "exit",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<(Result<(), ()>,)>::lift_args(params_slice, bytes))?;
            let res = store_data.exit(args.0)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <()>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <()>::arg_types();
    let mut result_ty = <Own<InputStreamResource>>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:cli/stdin@0.2.0#get-stdin");
    linker.func_new(
        "wasi:cli/stdin@0.2.0",
        "get-stdin",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<()>::lift_args(params_slice, bytes))?;
            let res = store_data.get_stdin()?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <Own<InputStreamResource>>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <()>::arg_types();
    let mut result_ty = <Own<OutputStreamResource>>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:cli/stdout@0.2.0#get-stdout");
    linker.func_new(
        "wasi:cli/stdout@0.2.0",
        "get-stdout",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<()>::lift_args(params_slice, bytes))?;
            let res = store_data.get_stdout()?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <Own<OutputStreamResource>>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <()>::arg_types();
    let mut result_ty = <Own<OutputStreamResource>>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:cli/stderr@0.2.0#get-stderr");
    linker.func_new(
        "wasi:cli/stderr@0.2.0",
        "get-stderr",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<()>::lift_args(params_slice, bytes))?;
            let res = store_data.get_stderr()?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <Own<OutputStreamResource>>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(i32,)>::arg_types();
    let mut result_ty = <()>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:cli/terminal-input@0.2.0#[resource-drop]terminal-input");
    linker.func_new(
        "wasi:cli/terminal-input@0.2.0",
        "[resource-drop]terminal-input",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<(i32,)>::lift_args(params_slice, bytes))?;
            let res = store_data.resource_drop_terminal_input(args.0)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <()>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <(i32,)>::arg_types();
    let mut result_ty = <()>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:cli/terminal-output@0.2.0#[resource-drop]terminal-output");
    linker.func_new(
        "wasi:cli/terminal-output@0.2.0",
        "[resource-drop]terminal-output",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<(i32,)>::lift_args(params_slice, bytes))?;
            let res = store_data.resource_drop_terminal_output(args.0)?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <()>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <()>::arg_types();
    let mut result_ty = <Option<Own<TerminalInputResource>>>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:cli/terminal-stdin@0.2.0#get-terminal-stdin");
    linker.func_new(
        "wasi:cli/terminal-stdin@0.2.0",
        "get-terminal-stdin",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<()>::lift_args(params_slice, bytes))?;
            let res = store_data.get_terminal_stdin()?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <Option<Own<TerminalInputResource>>>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <()>::arg_types();
    let mut result_ty = <Option<Own<TerminalOutputResource>>>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:cli/terminal-stdout@0.2.0#get-terminal-stdout");
    linker.func_new(
        "wasi:cli/terminal-stdout@0.2.0",
        "get-terminal-stdout",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<()>::lift_args(params_slice, bytes))?;
            let res = store_data.get_terminal_stdout()?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <Option<Own<TerminalOutputResource>>>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    let mut params_ty = <()>::arg_types();
    let mut result_ty = <Option<Own<TerminalOutputResource>>>::arg_types();
    let has_external_result = result_ty.len() > 1;
    if has_external_result {
        params_ty.push(ValType::I32);
        result_ty.clear();
    }

    println!("adding: wasi:cli/terminal-stderr@0.2.0#get-terminal-stderr");
    linker.func_new(
        "wasi:cli/terminal-stderr@0.2.0",
        "get-terminal-stderr",
        FuncType::new(params_ty, result_ty),
        move |mut caller, params, results| {
            let memory_pre = *caller.data().get_memory(memory_index);
            let (bytes, store_data) = memory_pre
                .memory
                .data_and_store_mut(caller.as_context_mut());

            let params_slice = if has_external_result {
                &params[0..(params.len() - 1)]
            } else {
                params
            };

            #[allow(unused)]
            let args = anyhow_result_to_wasmi(<()>::lift_args(params_slice, bytes))?;
            let res = store_data.get_terminal_stderr()?;
            let mut memory_filled = memory_pre.fill(caller);

            if has_external_result {
                let address = params[params.len() - 1].i32().unwrap() as usize;
                let range = address..(address + <Option<Own<TerminalOutputResource>>>::byte_size());
                anyhow_result_to_wasmi(res.lower_bytes(range, &mut memory_filled))?;
            } else {
                anyhow_result_to_wasmi(res.lower_args(results, &mut memory_filled))?;
            }

            Ok(())
        },
    )?;

    Ok(())
}

#[allow(unused)]
pub fn instantiate_root_world<D>(
    mut ctx: impl AsContextMut<Data = StoreData<D>>,
    component: &Component,
) -> Result<RootExports> {
    #[allow(unused_mut)]
    let mut linker = Linker::<StoreData<D>>::new(ctx.as_context().engine());
    let memory_index = ctx.as_context_mut().data_mut().next_memory_index();

    if component.is_wasi_p2() {
        add_wasi_p2_to_linker(ctx.as_context_mut(), &mut linker, memory_index)?;
    }

    add_root_to_linker(ctx.as_context_mut(), &mut linker, memory_index)?;

    let instance = linker.instantiate_and_start(ctx.as_context_mut(), &component.core_module)?;

    let memory = instance
        .get_memory(ctx.as_context(), "memory")
        .context("get memory")?;
    let cabi_realloc = instance
        .get_typed_func::<(i32, i32, i32, i32), i32>(ctx.as_context_mut(), "cabi_realloc")?;

    let memory_pre = MemoryAccessPre::new(memory, cabi_realloc);
    ctx.as_context_mut()
        .data_mut()
        .insert_memory(memory_index, memory_pre);

    Ok(RootExports {})
}
