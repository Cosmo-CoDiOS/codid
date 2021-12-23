//! Main executable for the `codictl` tool

use std::env;
use std::error::Error;

use clap::{App, Arg, ArgMatches};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn get_args() -> Result<ArgMatches<'static>, Box<dyn Error>> {
    let matches = App::new("codictl")
        .version(VERSION)
        .author("The Cosmo-CoDiOS Group")
        .about("Client to the codid server")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Verbosity level"),
        )
        .get_matches();

    Ok(matches.clone())
}

fn main() {
    let matches =
        get_args().expect("ERROR: Failed to get CLI arguments, this is bad!");

    let _min_log_level = match matches.occurrences_of("verbose") {
        0 => slog::Level::Info,
        1 => slog::Level::Debug,
        _ => slog::Level::Trace,
    };

    /* let log = setup_logging(min_log_level).expect("Could not setup logging."); */
}
