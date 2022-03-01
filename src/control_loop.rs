use std::path::Path;
use crate::State;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialOrd, PartialEq, Eq)]
pub enum ControlLoopError<'a> {
    NoSuchSocket(&'a Path),
    #[allow(dead_code)]
    InvalidMethod(String),
    #[allow(dead_code)]
    InvalidMethodArgs(Vec<String>),
}

#[allow(unreachable_code)]
pub(crate) fn enter_control_loop<'a>(
    s: &State,
    sock: &'a Path,
) -> Result<(), ControlLoopError<'a>> {
    // clone and lock state
    let log = s
        .lock()
        .unwrap()
        .log
        .clone()
        .new(o!("module" => "control_loop"));

    debug!(log, "Entering command loop...");

    // check socket exists
    // it should be initialized by the daemon module

    if !sock.exists() {
        // no such socket
        return Err(ControlLoopError::NoSuchSocket(sock));
    }

    loop {
        trace!(log, "Waiting for command...");
    }
}
