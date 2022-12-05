//! Modules for interfacing with the `/proc` FS special files provided by the Cosmo Linux kernel.

use std::fmt;
use std::error;
use std::fs;
use std::io::Write;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum ProcUtilError {
    Stm32ResetErr(std::io::Error),
    Stm32SetDownloadErr(std::io::Error),
    Stm32ResetDownloadErr(std::io::Error),
}

impl error::Error for ProcUtilError {}

impl fmt::Display for ProcUtilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            ProcUtilError::Stm32SetDownloadErr(_e) => write!(f, "Error toggling STM32 into download mode."),
            ProcUtilError::Stm32ResetDownloadErr(_e) => write!(f, "Error toggling STM32 out of download mode."),
            ProcUtilError::Stm32ResetErr(_e) => write!(f, "Error resetting the STM32.")
        }
    }
}

const AEON_RESET_STM32_PROC: &str = "/proc/AEON_RESET_STM32";
const AEON_STM32_DL_FW_PROC: &str = "/proc/AEON_STM32_DL_FW";

/// `hw_reset_stm32` flips the GPIO pins on the STM32, thus resetting `CoDi`. This is done
/// forcefully, and by doing it this way, `CoDi` has no way to power down itself. When `CoDiOS` is
/// running, as discovered during startup of the CLI, it will accept a 'safe power down' command to
/// avoid corruption to flash.
pub fn hw_reset_stm32() -> Result<(), ProcUtilError> {
    info!("Resetting CoDi...");

    trace!("Open fd for STM32 reset proc");
    let mut proc = fs::OpenOptions::new() // don't create, only write to special file
        .write(true)
        .append(false)
        .read(false)
        .create(false)
        .open(AEON_RESET_STM32_PROC)
        .unwrap();

    match proc.write_all("1".as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(ProcUtilError::Stm32ResetErr(e)),
    }

    debug!("Wait a little while....");
    thread::sleep(Duration::from_secs(2));

    info!("Starting CoDi again, please wait a moment...");

    match proc.write_all("0".as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(ProcUtilError::Stm32ResetErr(e)),
    }

    debug!("Wait for CoDi to start....");
    thread::sleep(Duration::from_secs(4));

    info!("CoDi should now be started."); // for CoDiOS, should we wait for a 'READY' signal?

    #[cfg(feature = "stock-codi")]
    {
        info!("Stock CoDi should now be showing the splash screen.");
        info!("In the event that CoDi does not boot, please wait for a bit, and/or \
            report the issue.");
    }

    Ok(())
}

/// `stm32_bootloader_dl` accepts one parameter (`in_out`) of the `bool` type. If it's `true`, then
/// it'll flip the GPIO pins that tell the STM32 to reboot to the bootloader. Likewise, if `in_out` is `false`,
/// then it'll flip the GPIO pins the other way to reboot to 'normal mode' of the STM32 firmware.
pub fn stm32_bootloader_dl(in_out: bool) -> Result<(), ProcUtilError> {
    trace!("Open fd for STM32 reset proc");

    let mut proc = fs::OpenOptions::new() // don't create, only write to special file
        .write(true)
        .append(false)
        .read(false)
        .create(false)
        .open(AEON_STM32_DL_FW_PROC)
        .unwrap();

    if in_out {
        // true, we're uploading (downloading from CoDi's PoV) firmware
        match proc.write_all("1".as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(ProcUtilError::Stm32SetDownloadErr(e)),
        }

        proc.write_all("1".as_ref())
            .expect("Unable to switch CoDi into DL mode!");
    } else {
        // false, we're not uploading to CoDi
        // reset to cmd mode
        match proc.write_all("0".as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(ProcUtilError::Stm32ResetDownloadErr(e)),
        }
    }

    Ok(())
}
