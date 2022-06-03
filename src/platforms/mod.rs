#[cfg(feature = "android")]
pub(crate) mod android;

#[cfg(feature = "gemian")]
pub(crate) mod gemian;

#[cfg(feature = "sailfish")]
pub(crate) mod sailfish;

#[cfg(feature = "ubports")]
pub(crate) mod ubports;

#[cfg(feature = "postmarketos")]
pub(crate) mod postmarketos;

pub(crate) mod common;
#[cfg(all(feature = "nixos", target_os = "linux"))]
pub(crate) mod nixos;
