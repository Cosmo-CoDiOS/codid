#[cfg(all(
    feature = "ubports",
    feature = "gemian",
    feature = "postmarketos",
    not(feature = "android"),
))]

pub(crate) mod contacts_dbus;
