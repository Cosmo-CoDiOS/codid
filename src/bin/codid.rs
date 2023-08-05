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
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use clap::{value_parser, Arg, ArgMatches, Command};
use config::{Config, Environment, File};

use codid::daemon::start;
use codid::StateStruct;

use anyhow::{Context, Result};

const ANDROID_CONF_PATH: &str =
    "/data/data/com.github.cosmo_codios.manager/codid/config.toml";

const SYSTEM_CONF_DEF_PATH: &str = "/etc/cosmo-codios/codid/config.toml";

mod errors {
    #[derive(Debug, thiserror::Error)]
    pub(crate) enum ConfigError {
        #[error("Config path does not exist.")]
        ConfigPathNonExistent,
        #[error("General configuration error.")]
        GeneralConfigError(#[source] config::ConfigError),
    }

    #[derive(Debug, thiserror::Error)]
    pub(crate) enum ArgsError {
        #[error("Could not get config path from Clap, including default")]
        ArgConfigPathGetError(#[source] clap::parser::MatchesError),
    }
}

fn load_config(cfg_file: &Path) -> Result<Config, errors::ConfigError> {
    debug!("Loading configuration, testing passed location for existence.");
    let path = if cfg_file.exists() {
        cfg_file.to_path_buf()
    } else {
        return Err(errors::ConfigError::ConfigPathNonExistent);
    };

    debug!("Configuration exists, load into memory.");

    let cfg = Config::builder()
        .add_source(File::from(path))
        .add_source(Environment::with_prefix("CODID"))
        .build()
        .map_err(errors::ConfigError::GeneralConfigError)?;

    Ok(cfg)
}

fn get_default_cfg_path() -> PathBuf {
    let xdg_dir = dirs::config_dir()
        .unwrap_or_default()
        .join(PathBuf::from("codid/config.toml"));

    let android_dir = PathBuf::from(ANDROID_CONF_PATH);
    let system_dir = PathBuf::from(SYSTEM_CONF_DEF_PATH);

    /* we don't handle readability here */
    if android_dir.exists() {
        return android_dir;
    } else if xdg_dir.exists() {
        return xdg_dir;
    }

    system_dir
}

fn get_args() -> ArgMatches {
    Command::new(env!("CARGO_BIN_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand_required(true)
        .arg(
            Arg::new("config")
                .long("config")
                .short('c')
                .value_name("FILE")
                .value_parser(value_parser!(PathBuf))
                .default_value(get_default_cfg_path().into_os_string())
                .help("Path to TOML configuration"),
        )
        .subcommand(Command::new("spawn").about("Starts the daemon."))
        .get_matches()
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    /* get initial `ArgMatches` */
    let args = get_args();

    env_logger::init();

    let cfg = load_config(args.try_get_one::<PathBuf>("config")
                        .map_err(errors::ArgsError::ArgConfigPathGetError)
                        .context("Error getting configuration path from Clap, including default")?
                        .unwrap())
        .context("Error parsing configuration file. Is it up to date, valid, and readable?")?;

    /* Initialise state */
    let state = Mutex::new(StateStruct { cfg });

    trace!("Loaded configuration into shared State.");

    /* interpret args */

    match args.subcommand() {
        Some(("spawn", _)) => {
            debug!("Handing over to daemon module...");
            start(&state).await.context("Failed to start daemon.")?;
        }
        _ => {
            unreachable!(); // this shouldn't be reached
        }
    }

    Ok(())
}
