//! Holder module for different Cosmo ROMs on the MTK chip.

#[cfg(all(feature = "android", target_os = "android"))]
pub(crate) mod android;

#[cfg(all(feature = "gemian", target_os = "linux"))]
pub(crate) mod gemian;

#[cfg(all(feature = "droidian", target_os = "linux"))]
pub(crate) mod droidian;

#[cfg(all(feature = "ubports", target_os = "linux"))]
pub(crate) mod ubports;

#[cfg(all(feature = "postmarketos", target_os = "linux"))]
pub(crate) mod postmarketos;

#[cfg(all(feature = "sailfishos", target_os = "linux"))]
pub(crate) mod sailfishos;

#[cfg(all(feature = "nixos", target_os = "linux"))]
pub(crate) mod nixos;

#[cfg(all(target_os = "linux", not(target_os = "android")))]
pub(crate) mod linux;

pub mod common;
