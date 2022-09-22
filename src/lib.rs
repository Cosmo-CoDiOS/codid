//! This module holds various bits and pieces for the `codid` daemon to run.
#![deny(
    warnings,
    missing_copy_implementations,
    unused_imports,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
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

use std::sync::{Arc, Mutex};

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
compile_error!("Target CPU not supported, please respecify!");

/// `StateStruct` is the shared state that the `State` type holds, using `Arc<Mutex<T>>`.
#[derive(Debug, Clone)]
pub struct StateStruct {
    /// This field holds the configuration struct for `codid`.
    pub cfg: config::Config,
}

/// `State` defines a custom type that holds `StateStruct` in an `Arc<Mutex<T>>`.
pub type State = Arc<Mutex<StateStruct>>;

pub mod codi_variants;
pub(crate) mod control_loop;
pub mod platforms;
pub mod rpc;

pub mod daemon {
    //! This is the module for the `codid` daemon.

    use std::path::Path;
    use std::thread;

    use crate::control_loop::{enter_control_loop, ControlLoopError};

    use super::State;

    /// Daemon entrypoint
    pub fn start(s: State) {
        info!("Hello, Cosmo!");

        debug!("Initializing daemon control loop...");

        let path = Path::new(
            s.clone()
                .lock()
                .expect("Unable to get a lock on the Config.")
                .cfg
                .get("rpc_socket")
                .unwrap_or("/tmp/codid.sock"),
        );

        // LAUNCH THREAD
        let ctrl_loop_thread = thread::Builder::new()
            .name("control_loop".to_string())
            .spawn(move || {
                match enter_control_loop(&s, &path.clone()) {
                    Ok(_) => (),
                    Err(e) => match e {
                        ControlLoopError::ServerStartError(_path) => {
                            error!("Could not start server.");
                            std::process::exit(1);
                        }
                    },
                }
            });


        info!("The Cosmo-CoDiOS daemon has now started.");
        info!("Running until asked to stop...");

        ctrl_loop_thread.unwrap().join().unwrap();
    }
}
