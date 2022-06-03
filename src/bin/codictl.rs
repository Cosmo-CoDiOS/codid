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

use std::env;
use std::sync::{Arc, Mutex};

use clap::{Arg, ArgMatches, Command};
use config::Config;
use slog::debug;

use codid::logging::setup_logging;
use codid::StateStruct;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn get_args() -> Option<ArgMatches> {
    let matches = Command::new("codictl")
        .version(VERSION)
        .author("The Cosmo-CoDiOS Group")
        .about("Client to the codid server")
        .subcommand_required(true)
        .arg(
            Arg::new("verbose")
                .short('v')
                .multiple_occurrences(true)
                .help("Verbosity level"),
        )
        .subcommand(Command::new("reset").about("Reset CoDi (reboot)"))
        .get_matches();

    Some(matches.clone())
}

fn main() {
    let matches =
        get_args().expect("ERROR: Failed to get CLI arguments, this is bad!");

    let min_log_level = match matches.occurrences_of("verbose") {
        0 => slog::Level::Info,
        1 => slog::Level::Debug,
        _ => slog::Level::Trace,
    };

    let log = setup_logging(min_log_level).expect("Could not setup logging.");

    /* Initialise state */
    let state = Arc::new(Mutex::new(StateStruct {
        log: log.clone(),
        cfg: Config::default(), // we don't use the config for the client, so let's specify a dummy
    }));

    match matches.subcommand() {
        Some(("reset", _)) => {
            debug!(log, "Handing over to daemon module...");
            codid::platforms::common::proc::hw_reset_stm32(&state);
        }
        _ => {
            unreachable!(); // this shouldn't be reached
        }
    }
}
