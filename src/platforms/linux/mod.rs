//! Generic Linux-targeted module for shared code.
#[cfg(all(
    feature = "ubports",
    feature = "gemian",
    feature = "droidian",
    feature = "postmarket",
    feature = "sailfish",
    feature = "nixos",
    not(feature = "android"),
))]

pub(crate) mod contacts_dbus;
