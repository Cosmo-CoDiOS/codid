//! Main executable for the `codictl` tool

use std::env;
use std::error::Error;

use clap::{App, Arg, ArgMatches, SubCommand};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn get_args() -> Result<ArgMatches<'static>, Box<dyn Error>> {
    let matches = App::new("codictl")
        .version(VERSION)
        .author("The codid Developers")
        .about("Client to the codid server")
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("Verbosity level"))
        .subcommand(SubCommand::with_name("spawn")
                        .about("Starts the daemon"))
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

   /* let log = setup_logging(min_log_level).expect("Could not setup logging."); */
}
