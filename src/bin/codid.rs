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
use std::sync::{Arc, Mutex};

use clap::{Arg, ArgMatches, Command};
use config::{Config, Environment, File};

use codid::daemon::start;
use codid::StateStruct;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn load_config(cfg_file: PathBuf) -> Option<Config> {
    let path = if cfg_file.exists() {
        Path::new(&cfg_file).to_path_buf()
    } else {
        return None
    };

    let cfg = Config::builder()
        .add_source(File::from(path))
        .add_source(Environment::with_prefix("CODI"))
        .build()
        .expect("Unable to construct Config struct");

    Some(cfg)
}

fn get_default_cfg_path() -> Option<PathBuf> {
    let xdg_dir = PathBuf::from(dirs::config_dir().unwrap().join(PathBuf::from("codid/config.toml")));
    let android_dir =  PathBuf::from("/data/data/com.github.cosmo_codios.codid.android/config.toml");

    println!("{:?}", xdg_dir.display());

    if xdg_dir.exists() {
        return Some(xdg_dir);
    } else if android_dir.exists() {
        return Some(android_dir)
    } else {
        return None
    }
}

fn get_args() -> ArgMatches {
    Command::new("codid")
        .version(VERSION)
        .author("The Cosmo-CoDiOS Group")
        .subcommand_required(true)
        .about("Cross-platform daemon-based interface to the Cosmo Communicator's cover display")
        .arg(Arg::new("config")
            .long("config")
            .short('c')
            .help("Path to TOML configuration"))
        .subcommand(Command::new("spawn")
            .about("Starts the daemon."))
        .get_matches()
}


fn main() {
    let args = get_args();
    env_logger::init();

    /* load config file */

    let cfg_path = match args.get_one::<PathBuf>("config") {
        Some(cfg_path) => PathBuf::from(cfg_path),
        None => get_default_cfg_path().expect("Unable to get configuration path from default logic. Likelihood is that config doesn't exist."),
    };

    let cfg = load_config(cfg_path)
        .expect("Error parsing configuration file. Check the validity.");

    /* Initialise state */
    let state = Arc::new(Mutex::new(StateStruct {
        cfg: cfg.clone(),
    }));

    trace!(
        "Loaded configuration into shared State."
    );

    // handle subcommands

    match args.subcommand() {
        Some(("spawn", _)) => {
            debug!("Handing over to daemon module...");
            start(state.clone());
        }
        _ => {
            unreachable!(); // this shouldn't be reached
        }
    }
}
