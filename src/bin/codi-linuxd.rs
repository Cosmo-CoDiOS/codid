//! Main executable for the `codi-linuxd` daemon.
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
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

use clap::{App, Arg, ArgMatches};

use codi_linuxd::daemon::start;
use codi_linuxd::logging::setup_logging;
use codi_linuxd::State;

use config::Config;
use futures::executor::block_on;

use slog::{debug, trace};

use std::env;
use std::error::Error;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn load_config(cfg_file: &str) -> Result<Config, Box<dyn Error>> {
    let path = std::path::Path::new(cfg_file);
    let cfg = Config::default()
        .merge(config::File::from(path))
        .unwrap()
        .merge(config::Environment::with_prefix("CODI_LINUXD"))
        .unwrap()
        .clone();

    Ok(cfg)
}

fn get_args() -> Result<ArgMatches<'static>, Box<dyn Error>> {
    let matches = App::new("codi-linuxd")
        .version(VERSION)
        .author("The codi-linuxd Developers")
        .about("Cross-platform interface to the Cosmo Communicator's cover display (CoDi)")
        .arg(Arg::with_name("config")
            .long("config")
            .short("c")
            .takes_value(true)
            .required(false)
            .help("Path to TOML configuration"))
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("Verbosity level"))
        .get_matches();

    Ok(matches.clone())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches =
        get_args().expect("ERROR: Failed to get CLI arguments, this is bad!");

    let min_log_level = match matches.occurrences_of("verbose") {
        0 => slog::Level::Info,
        1 => slog::Level::Debug,
        2 | _ => slog::Level::Trace,
    };

    let log = setup_logging(min_log_level);

    /* load config file */
    let default_cfg_path =
        format!("{}/.config/codi-linuxd/config.toml", env!("HOME")).clone();

    let cfg_path = matches.value_of("config").unwrap_or(&default_cfg_path);

    let cfg = load_config(cfg_path.clone()).unwrap();

    /* Initialise state */
    let state: State = State {
        log: log.clone(),
        cfg: cfg.clone(),
    };

    trace!(
        log,
        "Loaded configuration and (root) logger into shared State"
    );

    debug!(log, "Handover to daemon");
    block_on(start(state));

    Ok(())
}
