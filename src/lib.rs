//! This module holds various bits and pieces for the `codid` daemon to run.
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

#[macro_use]
extern crate slog;

/// This structure is for accessing shared fields in the daemon.
/// Should look into mutexes down the line.
#[derive(Debug, Clone)]
pub struct State {
    /// Field for holding a configuration object.
    pub cfg: config::Config,
    /// Field for accessing the base logger.
    pub log: slog::Logger,
}

pub mod logging;
pub(crate) mod platforms;
pub(crate) mod rpc;

pub mod daemon {
    //! This is the module for the `codid` daemon.

    use super::State;

    /// Daemon entrypoint
    pub async fn start(state: State) {
        let log = state.log;
        let _cfg = state.cfg; /* not used yet */

        info!(log, "Hello, Cosmo!");
    }
}
