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

#[macro_use]
extern crate log;

use std::env;
use std::path::PathBuf;
use std::sync::Mutex;

use clap::{Arg, ArgMatches, Command};
use config::{Config, Environment, File};

use codid::daemon::start;
use codid::StateStruct;

use anyhow::{Context, Result};

#[derive(Debug, thiserror::Error)]
enum ConfigError {
    #[error("Config path does not exist.")]
    ConfigPathNonExistent,
    #[error("General configuration error.")]
    GeneralConfigError(#[source] config::ConfigError),
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
const ANDROID_CONF_PATH: &str =
    "/data/data/com.github.cosmo_codios.manager/codid/config.toml";

fn load_config(cfg_file: &PathBuf) -> Result<Config, ConfigError> {
    debug!("Loading configuration, testing passed location for existence.");
    let path = if cfg_file.exists() {
        cfg_file.to_path_buf()
    } else {
        return Err(ConfigError::ConfigPathNonExistent);
    };

    debug!("Configuration exists, load into memory.");
    let cfg = Config::builder()
        .add_source(File::from(path))
        .add_source(Environment::with_prefix("CODID"))
        .build()
        .map_err(ConfigError::GeneralConfigError)?;

    return Ok(cfg);
}

fn get_default_cfg_path() -> Option<PathBuf> {
    let xdg_dir = dirs::config_dir()
        .unwrap()
        .join(PathBuf::from("codid/config.toml"));

    let android_dir = PathBuf::from(ANDROID_CONF_PATH);

    if xdg_dir.exists() {
        return Some(xdg_dir);
    } else if android_dir.exists() {
        return Some(android_dir);
    }

    None
}

fn get_args() -> ArgMatches {
    Command::new("codid")
        .version(VERSION)
        .author("The Cosmo-CoDiOS Group")
        .subcommand_required(true)
        .about("Cross-platform interface to the Cosmo Communicator's cover display")
        .arg(Arg::new("config")
            .long("config")
            .short('c')
            .help("Path to TOML configuration"))
        .subcommand(Command::new("spawn")
            .about("Starts the daemon."))
        .get_matches()
}

fn main() -> Result<(), std::error::Error> {
    /* get initial `ArgMatches` */
    let args = get_args();

    env_logger::init();

    /* load config file */

    let cfg_path = match args.get_one::<PathBuf>("config") {
        Some(cfg_path) => PathBuf::from(cfg_path),
        None => get_default_cfg_path().unwrap_or_default(),
    };

    let cfg = load_config(&cfg_path)
        .context("Error parsing configuration file. Check the validity of the config format.")?;

    /* Initialise state */
    let state = Mutex::new(StateStruct { cfg });

    trace!("Loaded configuration into shared State.");

    /* interpret args */

    match args.subcommand() {
        Some(("spawn", _)) => {
            debug!("Handing over to daemon module...");
            start(&state);
        }
        _ => {
            unreachable!(); // this shouldn't be reached
        }
    }

    Ok(())
}
