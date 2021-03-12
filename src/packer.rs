//! Module for serializing data for transmission over serial.
//! Soon to include deserialization functions.
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
#![allow(dead_code)]

#[derive(restruct_derive::Struct)]
#[fmt = ">B"]
pub(crate) struct U8Utility;

#[derive(restruct_derive::Struct)]
#[fmt = ">H"]
pub(crate) struct U16Utility;

#[derive(restruct_derive::Struct)]
#[fmt = ">I"]
pub(crate) struct U32Utility;

#[derive(restruct_derive::Struct)]
#[fmt = ">b"]
pub(crate) struct I8Utility;

#[derive(restruct_derive::Struct)]
#[fmt = ">h"]
pub(crate) struct I16Utility;

#[derive(restruct_derive::Struct)]
#[fmt = ">i"]
pub(crate) struct I32Utility;

impl U8Utility {
    pub(crate) fn encode(val: u8) -> <Self as restruct::Struct>::Packed {
        Self::pack((val,))
    }
}

impl U16Utility {
    pub(crate) fn encode(val: u16) -> <Self as restruct::Struct>::Packed {
        Self::pack((val,))
    }
}

impl U32Utility {
    pub(crate) fn encode(val: u32) -> <Self as restruct::Struct>::Packed {
        Self::pack((val,))
    }
}

impl I8Utility {
    pub(crate) fn encode(val: i8) -> <Self as restruct::Struct>::Packed {
        Self::pack((val,))
    }
}

impl I16Utility {
    pub(crate) fn encode(val: i16) -> <Self as restruct::Struct>::Packed {
        Self::pack((val,))
    }
}

impl I32Utility {
    pub(crate) fn encode(val: i32) -> <Self as restruct::Struct>::Packed {
        Self::pack((val,))
    }
}
