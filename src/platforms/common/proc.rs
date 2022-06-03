#![allow(dead_code)]

use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;

use crate::State;

const AEON_RESET_STM32_PROC: &str = "/proc/AEON_RESET_STM32";
const AEON_STM32_DL_FW_PROC: &str = "/proc/AEON_STM32_DL_FW";

pub fn hw_reset_stm32(s: &State) {
    let log = s
        .lock()
        .expect("Failed to get a lock on the logger.")
        .log
        .new(o!("module" => "common_proc_stm32_reset"));

    info!(log, "Resetting CoDi...");

    trace!(log, "Open fd for STM32 reset proc");
    let mut proc = OpenOptions::new() // don't create, only write to special file
        .write(true)
        .read(false)
        .create(false)
        .open(&AEON_RESET_STM32_PROC)
        .unwrap();

    proc.write_all("1".as_ref()).expect("Unable to reset CoDi.");

    debug!(log, "Wait a little while....");
    thread::sleep(Duration::from_secs(2));

    info!(log, "Starting CoDi again, please wait a moment...");
    proc.write_all("0".as_ref()).expect("Unable to start CoDi.");

    debug!(log, "Wait for CoDi to start....");
    thread::sleep(Duration::from_secs(4));

    info!(log, "CoDi should now be started."); // for CoDiOS, should we wait for a 'READY' signal?

    #[cfg(feature = "stock-codi")]
    {
        info!(log, "[Stock] CoDi should now be showing the splash screen.");
        info!(log, "In the event that CoDi does not boot, please wait for a bit, and/or \
            report the issue.");
    }

    trace!(log, "Dropping the lock on the Logger");

    // release lock
    drop(log);
}

pub fn stm32_bootloader_dl(in_out: bool, s: &State) {
    let log = s
        .lock()
        .expect("Failed to get a lock on the logger.")
        .log
        .new(o!("module" => "common_proc_stm32_fw_dl"));

    trace!(log, "Open fd for STM32 reset proc");

    let mut proc = OpenOptions::new() // don't create, only write to special file
        .write(true)
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

    trace!(log, "Dropping the lock on the Logger");
    drop(log);
}
