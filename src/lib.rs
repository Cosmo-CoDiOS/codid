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

// check for CoDiOS or stock CoDi feature enablement
#[cfg(not(any(feature = "stock-codi", feature = "codios-codi")))]
compile_error!("ONE variant of CoDi required as a feature, please respecify!");

// check for at least one ROM target
#[cfg(not(any(
    feature = "sailfish",
    feature = "android",
    feature = "ubports",
    feature = "gemian",
    feature = "postmarketos"
)))]
compile_error!(
    "At least ONE Cosmo ROM is required as a feature, please specify!"
);

#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
compile_error!("CPU I was asked to target isn't supported, try again!");

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
