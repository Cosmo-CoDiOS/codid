//! This module holds various bits and pieces for the `codid` daemon to run.
#![deny(
    warnings,
    missing_copy_implementations,
    unused_imports,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

#[macro_use]
extern crate log;

use std::sync::Mutex;

// check for CoDiOS or stock CoDi feature enablement
#[cfg(not(any(feature = "stock-codi", feature = "codios-codi")))]
compile_error!("ONE variant of CoDi required as a feature, please respecify!");

// check for at least one ROM target
#[cfg(not(any(
    feature = "android",
    feature = "ubports",
    feature = "gemian",
    feature = "droidian",
    feature = "postmarketos",
    feature = "sailfishos",
    feature = "nixos"
)))]
compile_error!(
    "At least ONE Cosmo ROM is required as a feature, please specify!"
);

#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
compile_error!("Target CPU not supported, please respecify!");

#[cfg(not(any(target_os = "linux", target_os = "android")))]
compile_error!("Build target not Linux OR Android, refusing to build!");

/// `StateStruct` is the shared state that the `State` type holds, using `Mutex<T>`.
#[derive(Debug, Clone)]
pub struct StateStruct {
    /// This field holds the configuration struct for `codid`.
    pub cfg: config::Config,
}

/// `State` defines a custom type that holds `StateStruct` in an `Mutex<T>`.
pub type State = Mutex<StateStruct>;

pub(crate) mod codi_event_loop;
pub(crate) mod codid_control_loop;
pub(crate) mod codid_event_loop;
pub mod platforms;

pub mod daemon {
    //! This is the module for the `codid` daemon.

    use super::State;

    /// Daemon entrypoint
    pub fn start(_s: &State) {
        info!("Hello, Cosmo!");

        info!("The Cosmo-CoDiOS daemon has now started.");
        info!("Running until asked to stop...");
    }
}
