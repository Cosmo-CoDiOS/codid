#![allow(dead_code)]

use crate::State;
use std::path::Path;

use anyhow::Result;
use thiserror::Error;
use std::io;
use jsonrpc_ipc_server::jsonrpc_core::IoHandler;
use jsonrpc_ipc_server::ServerBuilder;

/// `ControlLoopError` is an enum of different `Error` variants, backed by `anyhow` and `thiserror`.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Error)]
pub enum ControlLoopError {
    /// Returned when the JSON-RPC method called encounters an error.
    /// This also returns an error over JSON-RPC socket, but logs in the daemon as well.
    #[error("Execution of method errored.")]
    MethodExecutionError,
    /// Returned when transforming the `Path` to a `&str` fails.
    #[error("Error transforming `Path` to `&str`")]
    SocketPathTransformError,
    /// Generic error, derived from `std::io::Error` when the server fails to start.
    #[error("Server error")]
    ServerError {
        #[from]
        source: io::Error
    }
}

type ControlLoopResult = Result<(), ControlLoopError>;

pub(crate) fn enter_control_loop<'a>(
    _s: &State,
    sock: &'a Path,
) -> ControlLoopResult {
    debug!("Entering command loop...");

    let io = IoHandler::new();
    let sock_path_str = match sock.to_str() {
        Some(t) => t,
        None => return Err(ControlLoopError::SocketPathTransformError),
    };

    let server = ServerBuilder::new(io)
        .start(sock_path_str)
        .map_err(|source| ControlLoopError::ServerError { source })?;

    server.wait();
    Ok(())
}
