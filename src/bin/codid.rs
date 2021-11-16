//! Main executable for the `codid` daemon.

use std::env;
use std::error::Error;

use clap::{App, Arg, ArgMatches, SubCommand};
use config::Config;
use futures::executor::block_on;
use slog::{debug, trace};

use codid::daemon::start;
use codid::logging::setup_logging;
use codid::State;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn load_config(cfg_file: &str) -> Result<Config, Box<dyn Error>> {
    let path = std::path::Path::new(cfg_file);
    let cfg = Config::default()
        .merge(config::File::from(path))
        .unwrap()
        .merge(config::Environment::with_prefix("COSMO_CODID_CONFIG_PATH"))
        .unwrap()
        .clone();

    Ok(cfg)
}

fn get_args() -> Result<ArgMatches<'static>, Box<dyn Error>> {
    let matches = App::new("codid")
        .version(VERSION)
        .author("The codid Developers")
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
        .subcommand(SubCommand::with_name("spawn")
                        .about("Starts the daemon"))
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
        _ => slog::Level::Trace,
    };

    let log = setup_logging(min_log_level).expect("Could not setup logging.");

    /* load config file */

    let cfg_path = matches.value_of("config")
        .expect("Configuration file not specified. Try specifying the configuration file.");

    let cfg = load_config(&cfg_path)
        .expect("Error parsing configuration file. Check the validity.");

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
