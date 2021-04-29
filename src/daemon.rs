//! This is the module for the `cosmo-codi-d` daemon.

use super::State;

/// Daemon entrypoint
pub async fn start(state: State) {
    let log = state.log;
    let _cfg = state.cfg; /* not used yet */

    debug!(log, "Daemon initialising");
    debug!(log, "Daemon initialised");
    info!(log, "Hello, Cosmo!");
}
