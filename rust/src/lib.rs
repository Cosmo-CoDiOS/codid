//! This module holds various bits and pieces for the `codi-linuxd` daemon to run.
#![feature(const_fn)]
#![feature(const_fn_transmute)]
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

pub mod protocol;
pub(crate) mod packer;
