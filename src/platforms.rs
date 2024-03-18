//! Holder module for different Cosmo ROMs on the MTK chip.

#[cfg(all(feature = "android", target_os = "android"))]
pub mod android;

#[cfg(all(feature = "gemian", target_os = "linux"))]
pub mod gemian;

#[cfg(all(feature = "droidian", target_os = "linux"))]
pub mod droidian;

#[cfg(all(feature = "ubports", target_os = "linux"))]
pub mod ubports;

#[cfg(all(feature = "postmarketos", target_os = "linux"))]
pub mod postmarketos;

#[cfg(all(feature = "sailfishos", target_os = "linux"))]
pub mod sailfishos;

#[cfg(all(feature = "nixos", target_os = "linux"))]
pub mod nixos;

#[cfg(all(feature = "linux", not(target_os = "android")))]
pub mod linux;

pub mod common;
