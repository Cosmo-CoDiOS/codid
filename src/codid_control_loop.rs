//! Control loop managed by Tokio for interacting with `codid`.

use anyhow::Result;
use thiserror::Error;

/// `ControlLoopError` is an enum of different `Error` variants, backed by `anyhow` and `thiserror`.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error, Eq, PartialEq)]
pub enum ControlLoopError {
    /// Returned when the JSON-RPC method called encounters an error.
    /// This also returns an error over JSON-RPC socket, but logs in the daemon as well.
    #[error("Execution of method errored.")]
    MethodExecutionFailure,
    /// Returned when transforming the `Path` to a `&str` yields `None`.
    #[error("Transforming `Path` to `&str` yielded `None`.")]
    SocketPathTransformFailure,
}

pub type ControlLoopResult<T, E = ControlLoopError> = Result<T, E>;

pub fn enter_control_loop() -> ControlLoopResult<()> {
    Ok(())
}
