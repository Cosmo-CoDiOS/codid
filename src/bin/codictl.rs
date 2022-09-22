//! Main executable for the `codictl` tool
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
extern crate log;

use std::env;
use std::sync::{Arc, Mutex};

use clap::{Arg, ArgMatches, Command};
use config::Config;

use codid::StateStruct;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn get_args() -> Option<ArgMatches> {
    let matches = Command::new("codictl")
        .version(VERSION)
        .author("The Cosmo-CoDiOS Group")
        .about("Client to the codid server")
        .subcommand_required(true)
        .subcommand(Command::new("reset").about("Reset CoDi (reboot)"))
        .get_matches();

    Some(matches.clone())
}

fn main() {
    let args = get_args();
    env_logger::init();

    /* Initialise state */
    let state = Arc::new(Mutex::new(StateStruct {
        cfg: Config::default(), // we don't use the config for the client, so let's specify a dummy
    }));

    match matches.subcommand() {
        Some(("reset", _)) => {
            debug!("Handing over to daemon module...");
            codid::platforms::common::proc::hw_reset_stm32(&state);
        }
        _ => {
            unreachable!(); // this shouldn't be reached
        }
    }
}
