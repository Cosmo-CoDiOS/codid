//! Modules for interfacing with the `/proc` FS special files provided by the Cosmo Linux kernel.

use std::fs;
use std::io::Write;
use std::thread;
use std::time::Duration;

const AEON_RESET_STM32_PROC: &str = "/proc/AEON_RESET_STM32";
const AEON_STM32_DL_FW_PROC: &str = "/proc/AEON_STM32_DL_FW";

/// `hw_reset_stm32` flips the GPIO pins on the STM32, thus resetting `CoDi`. This is done
/// forcefully, and by doing it this way, `CoDi` has no way to power down itself. When `CoDiOS` is
/// running, as discovered during startup of the CLI, it will accept a 'safe power down' command to
/// avoid corruption to flash.
pub fn hw_reset_stm32() {
    info!("Resetting CoDi...");

    trace!("Open fd for STM32 reset proc");
    let mut proc = fs::OpenOptions::new() // don't create, only write to special file
        .write(true)
        .append(false)
        .read(false)
        .create(false)
        .open(&AEON_RESET_STM32_PROC)
        .unwrap();

    proc.write_all("1".as_ref()).expect("Unable to reset CoDi.");

    debug!("Wait a little while....");
    thread::sleep(Duration::from_secs(2));

    info!("Starting CoDi again, please wait a moment...");
    proc.write_all("0".as_ref()).expect("Unable to start CoDi.");

    debug!("Wait for CoDi to start....");
    thread::sleep(Duration::from_secs(4));

    info!("CoDi should now be started."); // for CoDiOS, should we wait for a 'READY' signal?

    #[cfg(feature = "stock-codi")]
    {
        info!("Stock CoDi should now be showing the splash screen.");
        info!("In the event that CoDi does not boot, please wait for a bit, and/or \
            report the issue.");
    }

    trace!("Dropping the lock on the Logger");
}

/// `stm32_bootloader_dl` accepts one parameter (`in_out`) of the `bool` type. If it's `true`, then
/// it'll flip the GPIO pins that tell the STM32 to reboot to the bootloader. Likewise, if `in_out` is `false`,
/// then it'll flip the GPIO pins the other way to reboot to 'normal mode' of the STM32 firmware.
pub fn stm32_bootloader_dl(in_out: bool) {
    trace!("Open fd for STM32 reset proc");

    let mut proc = fs::OpenOptions::new() // don't create, only write to special file
        .write(true)
        .append(false)
        .read(false)
        .create(false)
        .open(&AEON_STM32_DL_FW_PROC)
        .unwrap();

    if in_out {
        // true, we're uploading (downloading from CoDi's PoV) firmware
        proc.write_all("1".as_ref())
            .expect("Unable to switch CoDi into DL mode!");
    } else {
        // false, we're not uploading to CoDi
        // reset to cmd mode
        proc.write_all("0".as_ref())
            .expect("Unable to reset CoDi to normal comms mode.");
    }
}
