//! Main executable for the `codid` daemon.
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
use std::env::var;
use std::path::Path;
use std::sync::{Arc, Mutex};

use clap::{Arg, ArgMatches, Command};
use config::{Config, Environment, File};
use slog::{debug, trace};

use codid::daemon::start;
use codid::logging::setup_logging;
use codid::StateStruct;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn load_config(cfg_file: &str) -> Option<Config> {
    let path = if Path::new(cfg_file).exists() {
        Path::new(&cfg_file).to_path_buf()
    } else {
        let xdg_cfg_home = var("XDG_CONFIG_HOME").unwrap_or(format!(
            "{}/{}",
            var("HOME").unwrap(),
            "/.config"
        ));
        Path::new(&xdg_cfg_home).join("codid.toml").to_path_buf()
    };

    let cfg = Config::builder()
        .add_source(File::from(path))
        .add_source(Environment::with_prefix("CODI"))
        .build()
        .expect("Unable to construct Config struct");

    Some(cfg)
}

fn get_args() -> Option<ArgMatches> {
    let matches = Command::new("codid")
        .version(VERSION)
        .author("The Cosmo-CoDiOS Group")
        .subcommand_required(true)
        .about("Cross-platform daemon-based interface to the Cosmo Communicator's cover display")
        .arg(Arg::new("config")
            .long("config")
            .short('c')
            .takes_value(true)
            .help("Path to TOML configuration"))
        .arg(Arg::new("verbose")
            .short('v')
            .multiple_occurrences(true)
            .help("Verbosity level"))
        .subcommand(Command::new("spawn")
            .about("Starts the daemon."))
        .get_matches();

    Some(matches)
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

    /* load config file */

    let cfg_path = matches.value_of("config").unwrap_or_default();

    let cfg = load_config(&cfg_path)
        .expect("Error parsing configuration file. Check the validity.");

    /* Initialise state */
    let state = Arc::new(Mutex::new(StateStruct {
        log: log.clone(),
        cfg: cfg.clone(),
    }));

    trace!(
        log,
        "Loaded configuration and (root) logger into shared State."
    );

    // handle subcommands

    match matches.subcommand() {
        Some(("spawn", _)) => {
            debug!(log, "Handing over to daemon module...");
            start(state.clone());
        }
        _ => {
            unreachable!(); // this shouldn't be reached
        }
    }
}
