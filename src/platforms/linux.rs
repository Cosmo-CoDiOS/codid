//! Generic Linux-targeted module for shared code.

#[cfg(all(feature = "linux", not(target_os = "android")))]
pub mod contacts_eds;
