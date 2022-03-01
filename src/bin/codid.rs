//! Main executable for the `codid` daemon.

use std::env;
use std::env::var;
use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex};

use clap::{Arg, ArgMatches, Command};
use config::Config;
use slog::{debug, error, trace};

use codid::daemon::start;
use codid::logging::setup_logging;
use codid::StateStruct;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn load_config(cfg_file: &str) -> Result<Config, Box<dyn Error>> {
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

    let cfg = Config::new()
        .merge(config::File::from(path))
        .expect("Config doesn't exist!")
        .merge(config::Environment::with_prefix("CODID"))
        .expect("Unable to open environment variable for config overriding!")
        .clone();

    Ok(cfg)
}

fn get_args() -> Result<ArgMatches, Box<dyn Error>> {
    let matches = Command::new("codid")
        .version(VERSION)
        .author("The Cosmo-CoDiOS Group")
        .about("Cross-platform daemon-based interface to the Cosmo Communicator's cover display")
        .arg_required_else_help(true)
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

    Ok(matches)
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
            error!(log, "Unsupported subcommand. Use `--help`."); // clap takes care of this, though
            unreachable!();
        }
    }
}
