//! Control loop managed by Tokio for controlling the CoDi daemon.
#![allow(dead_code)]

use crate::State;

use anyhow::Result;
use futures::future;
use thiserror::Error;

/// `ControlLoopError` is an enum of different `Error` variants, backed by `anyhow` and `thiserror`.
#[allow(clippy::module_name_repetitions, clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum ControlLoopError {
    /// Returned when the JSON-RPC method called encounters an error.
    /// This also returns an error over JSON-RPC socket, but logs in the daemon as well.
    #[error("Execution of method errored.")]
    MethodExecutionError,
    /// Returned when transforming the `Path` to a `&str` yields `None`.
    #[error("Transforming `Path` to `&str` yielded `None`.")]
    SocketPathTransformError,
}

pub type ControlLoopResult = Result<(), ControlLoopError>;

pub(crate) async fn enter_control_loop(_s: &State) -> ControlLoopResult {
    debug!("Entering command loop...");

    future::ok(()).await
}
