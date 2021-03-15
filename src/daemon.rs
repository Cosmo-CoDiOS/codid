//! This is the module for the `codi-linuxd` daemon.

use super::State;

/// Daemon entrypoint
pub async fn start(state: State) {
    let log = state.log;
    let cfg = state.cfg;

    debug!(log, "Daemon initialising");
    debug!(log, "Daemon initialised");
    info!(log, "Hello, Cosmo!");
}
