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

use clap::{ArgMatches, Command};
use config::Config;

use codid::StateStruct;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn get_args() -> ArgMatches {
    Command::new("codictl")
        .version(VERSION)
        .author("The Cosmo-CoDiOS Group")
        .about("Client to the codid server")
        .subcommand_required(true)
        .subcommand(Command::new("reset").about("Reset CoDi (reboot)"))
        .get_matches()
}

fn main() {
    let args = get_args();
    env_logger::init();

    /* Initialise state */
    let _state = Arc::new(Mutex::new(StateStruct {
        cfg: Config::default(), // we don't use the config for the client, so let's specify a dummy
    }));

    match args.subcommand() {
        Some(("reset", _)) => {
            debug!("Handing over to daemon module...");
            codid::platforms::common::proc::hw_reset_stm32();
        }
        Some(("enter-bootloader", _)) => {
            codid::platforms::common::proc::stm32_bootloader_dl(true);
        }
        Some(("exit-bootloader", _)) => {
            codid::platforms::common::proc::stm32_bootloader_dl(false);
        }

        _ => {
            unreachable!(); // this shouldn't be reached
        }
    }
}
