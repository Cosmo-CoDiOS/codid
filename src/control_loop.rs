#![allow(dead_code)]

use crate::State;
use std::path::Path;
use std::result::Result;

use jsonrpc_ipc_server::jsonrpc_core::IoHandler;
use jsonrpc_ipc_server::ServerBuilder;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialOrd, PartialEq, Eq)]
pub enum ControlLoopError<'a> {
    ServerStartError(&'a Path),
}

type ControlLoopResult<'a> = Result<(), ControlLoopError<'a>>;

pub(crate) fn enter_control_loop<'a>(
    _s: &State,
    sock: &'a Path,
) -> ControlLoopResult<'a> {
    debug!("Entering command loop...");

    let io = IoHandler::new();

    let server = match ServerBuilder::new(io).start(sock.to_str().unwrap()) {
        Ok(s) => s,
        Err(_e) => return Err(ControlLoopError::ServerStartError(sock)),
    };

    server.wait();
    Ok(())
}
