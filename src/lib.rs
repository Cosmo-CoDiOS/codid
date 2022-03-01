//! This module holds various bits and pieces for the `codid` daemon to run.
#![deny(
warnings,
missing_copy_implementations,
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
extern crate slog;

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
    /// This field is the root `Logger` instance for `codid`.
    pub log: slog::Logger,
}

/// `State` defines a custom type that holds `StateStruct` in an `Arc<Mutex<T>>`.
pub type State = Arc<Mutex<StateStruct>>;

pub(crate) mod control_loop;
pub mod logging;
pub(crate) mod platforms;
pub mod rpc;

pub mod daemon {
    //! This is the module for the `codid` daemon.

    use std::path::Path;
    use std::thread;

    use crate::control_loop::{ControlLoopError, enter_control_loop};

    use super::State;

    /// Daemon entrypoint
    pub fn start(s: State) {
        let log = s
            .lock()
            .expect("Unable to get a lock on the Logger")
            .log
            .new(o!("module" => "daemon"));

        info!(log, "Hello, Cosmo!");
        debug!(log, "Initializing daemon control loop...");

        let path = Path::new("/run/user/1000/codi.sock");

        // LAUNCH THREAD
        let ctrl_loop_thread = thread::Builder::new()
            .name("control_loop".to_string())
            .spawn(move || {
                let log = s
                    .lock()
                    .expect("Unable to get a lock on the `Logger`")
                    .log
                    .new(o!("thread" => "control_loop_thread"));

                match enter_control_loop(&s, &path) {
                    Ok(_) => (),
                    Err(e) => match e {
                        ControlLoopError::NoSuchSocket(_path) => {
                            error!(log, "Control loop returned `NoSuchSocket` error - this is \
                            unrecoverable!");
                            error!(log, "Helpful information: Socket path value: `{}`",
                                path
                                .clone() // hacky fix to solve borrowing
                                .to_str()
                                .expect("Unable to get socket path to `&str` for debugging info!"));
                            std::process::exit(1);
                        }
                        ControlLoopError::InvalidMethod(method) => {
                            warn!(log, "Control loop returned `InvalidMethod` error. This isn't
                            a failure, but merely a invalid JSON-RPC request referencing a
                            unknown method. Continuing loop.");
                            warn!(log, "Helpful information: Invalid method value: `{}`", method);
                        }
                        ControlLoopError::InvalidMethodArgs(args) => {
                            warn!(log, "Control loop returned `InvalidMethodArgs` error. This \
                            isn't a failure, but merely an invalid JSON-RPC request referencing \
                            unknown method arguments. Continuing loop.");
                            warn!(log, "Helpful information: Invalid method args: `{:?}`", args);
                        }
                    }
                }
            });

        info!(log, "The Cosmo-CoDiOS daemon has now started.");
        info!(log, "Running until asked to stop...");

        ctrl_loop_thread.unwrap().join().unwrap();
    }
}
