//! Generic Linux-targeted module for shared code.

#[cfg(all(target_os = "linux", not(target_os = "android")))]
pub(crate) mod contacts_eds;
