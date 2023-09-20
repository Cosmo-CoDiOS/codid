//! Modules for interfacing with the `/proc` FS special files provided by the Cosmo Linux kernel.
#![allow(dead_code)]

use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use log::{info,trace,debug};

/// `ProcUtilError` is an enum of different `Error` types and reasons.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, thiserror::Error)]
pub enum ProcUtilError {
    /// `Stm32ResetErr` is an error returned when we're unable to write to the 'special file' for
    /// rebooting the STM32.
    #[error("Unable to reset the STM32.")]
    Stm32ResetErr(#[source] io::Error),
    /// `Stm32SetDownloadErr` is an error returned when `codid` is unable to write to the 'special
    /// file' for resetting the STM32 into 'bootloader mode'.
    #[error("Unable to bring the STM32 into Download mode.")]
    Stm32SetDownloadErr(#[source] io::Error),
    /// `Stm32ResetDownloadErr` is an error returned when we're unable to write to the 'special
    /// file' for resetting the STM32 into 'user mode'.
    #[error("Unable to bring the STM32 out of Download mode.")]
    Stm32ResetDownloadErr(#[source] io::Error),
    /// `Stm32WakeErr` is an error returned when we're unable to write to the 'special
    /// file' for resetting the STM32 into 'user mode'.
    #[error("Unable to wake the STM32.")]
    Stm32WakeErr(#[source] io::Error),
    /// `Stm32ProcIoError` is an error returned when initialising the file descriptor.
    #[error("Unable to setup a file descriptor for special /proc file.")]
    Stm32ProcIoErr(#[source] io::Error),
}

/// `ProcUtilResult` acts as an abstraction over `anyhow` and `thiserror`, used for handling errors
/// produced by the `crate::platforms::common::proc` module.
pub type ProcUtilResult = anyhow::Result<(), ProcUtilError>;

const AEON_RESET_STM32_PROC: &str = "/proc/AEON_RESET_STM32";
const AEON_STM32_DL_FW_PROC: &str = "/proc/AEON_STM32_DL_FW";
const AEON_WAKE_STM32_PROC: &str = "/proc/AEON_WAKE_STM32";

/// `stm32_reset` flips the GPIO pins on the STM32, thus resetting `CoDi`.
pub fn stm32_reset() -> ProcUtilResult {
    info!("Resetting CoDi...");

    trace!("Open fd for STM32 reset proc");
    let mut proc = open_proc_file(AEON_RESET_STM32_PROC)?;

    proc.write_all("1".as_bytes())
        .map_err(ProcUtilError::Stm32ResetErr)?;

    debug!("Wait a little while....");
    thread::sleep(Duration::from_secs(2));

    info!("Starting CoDi again, please wait a moment...");

    proc.write_all("0".as_bytes())
        .map_err(ProcUtilError::Stm32ResetErr)?;

    debug!("Wait for CoDi to start....");
    thread::sleep(Duration::from_secs(4));

    info!("CoDi should now be started."); // for CoDiOS, should we wait for a 'READY' signal?

    Ok(())
}

/// `stm32_wake` flips the GPIO pins on the STM32, thus waking up `CoDi` (IRQ)
pub fn stm32_wake() -> ProcUtilResult {
    info!("Waking CoDi...");

    let mut proc = open_proc_file(AEON_WAKE_STM32_PROC)?;

    proc.write_all("1".as_bytes())
        .map_err(ProcUtilError::Stm32WakeErr)?;

    debug!("Wait a little while....");
    thread::sleep(Duration::from_secs(2));

    proc.write_all("0".as_bytes())
        .map_err(ProcUtilError::Stm32WakeErr)?;

    Ok(())
}

/// `stm32_bootloader_dl` accepts one `bool` parameter (`in_out`).
///
/// If `true`, then it'll flip the GPIO pins that instruct the STM32 to reboot to the bootloader.
///
/// Likewise, if `in_out` is `false`, then it'll flip the GPIO pins the other way to reboot to
/// 'user mode' of the STM32 firmware.
pub fn stm32_bootloader_dl(in_out: bool) -> ProcUtilResult {
    trace!("Open fd for STM32 reset proc");

    let mut proc = open_proc_file(AEON_STM32_DL_FW_PROC)?;

    if in_out {
        // true, we're uploading (downloading from CoDi's PoV) firmware
        proc.write_all("1".as_bytes())
            .map_err(ProcUtilError::Stm32SetDownloadErr)?;
    } else {
        // false, we're not uploading to CoDi
        // reset to cmd mode
        proc.write_all("0".as_bytes())
            .map_err(ProcUtilError::Stm32ResetDownloadErr)?;
    }

    Ok(())
}

fn open_proc_file(file: &str) -> Result<File, ProcUtilError> {
    fs::OpenOptions::new()
        .write(true)
        .append(false)
        .read(false)
        .create(false)
        .open(file)
        .map_err(ProcUtilError::Stm32ProcIoErr)
}
