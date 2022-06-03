#[cfg(feature = "android")]
pub(crate) mod android;

#[cfg(all(feature = "gemian", target_os = "linux"))]
pub(crate) mod gemian;

#[cfg(all(feature = "sailfish", target_os = "linux"))]
pub(crate) mod sailfish;

#[cfg(all(feature = "ubports", target_os = "linux"))]
pub(crate) mod ubports;

#[cfg(all(feature = "postmarketos", target_os = "linux"))]
pub(crate) mod postmarketos;

#[cfg(all(feature = "nixos", target_os = "linux"))]
pub(crate) mod nixos;

#[cfg(all(target_os = "linux"))]
pub(crate) mod linux;

pub mod common;
