//! Main executable for the `codictl` tool
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

use anyhow::Context;

use std::env;

use clap::{ArgMatches, Command};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn get_args() -> ArgMatches {
    Command::new("codictl")
        .version(VERSION)
        .author("The Cosmo-CoDiOS Group")
        .about("Scriptable client to the codid interface")
        .subcommand_required(true)
        .subcommand(Command::new("reset-stm32").about("Reset the STM32"))
        .subcommand(
            Command::new("enter-stm32-bootloader")
                .about("Tell the STM32 to enter bootloader mode"),
        )
        .subcommand(
            Command::new("exit-stm32-bootloader")
                .about("Tell the STM32 to exit bootloader mode, and reset"),
        )
        .subcommand(
            Command::new("shutdown-codid").about("Shutdown the CoDi daemon."),
        )
        .get_matches()
}

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let args = get_args();

    match args.subcommand() {
        Some(("reset-stm32", _)) => {
            codid::platforms::common::proc::stm32_reset()
                .context("Unable to reset the STM32 at this time.")?;
        }
        Some(("enter-stm32-bootloader", _)) => {
            codid::platforms::common::proc::stm32_bootloader_dl(true).context(
                "Unable to tell the STM32 to enter into download mode.",
            )?;
        }
        Some(("exit-stm32-bootloader", _)) => {
            codid::platforms::common::proc::stm32_bootloader_dl(false)
                .context("Unable to tell the STM32 to exit download mode.")?;
        }
        _ => {
            unreachable!(); // this shouldn't be reached
        }
    }

    Ok(())
}
