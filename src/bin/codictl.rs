//! Main executable for the `codictl` tool

use std::env;
use std::error::Error;
use std::sync::{Arc, Mutex};

use clap::{Arg, ArgMatches, Command};
use config::Config;

use codid::logging::setup_logging;
use codid::StateStruct;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn get_args() -> Result<ArgMatches, Box<dyn Error>> {
    let matches = Command::new("codictl")
        .version(VERSION)
        .author("The Cosmo-CoDiOS Group")
        .about("Client to the codid server")
        .arg(
            Arg::new("verbose")
                .short('v')
                .multiple_occurrences(true)
                .help("Verbosity level"),
        )
        .get_matches();

    Ok(matches.clone())
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
    let _state = Arc::new(Mutex::new(StateStruct {
        log: log.clone(),
        cfg: Config::default(), // we don't use the config for the client, so let's specify a dummy
    }));
}
