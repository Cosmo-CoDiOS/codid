//! Module for serializing data for transmission over serial.
//! Soon to include deserialization functions.
#![allow(dead_code)]

#[derive(restruct_derive::Struct)]
#[fmt = ">B"]
pub(crate) struct UnsignedCharTransformer;

#[derive(restruct_derive::Struct)]
#[fmt = ">H"]
pub(crate) struct UnsignedShortTransformer;

#[derive(restruct_derive::Struct)]
#[fmt = ">I"]
pub(crate) struct UnsignedIntegerTransformer;

#[derive(restruct_derive::Struct)]
#[fmt = ">b"]
pub(crate) struct SignedCharTransformer;

#[derive(restruct_derive::Struct)]
#[fmt = ">h"]
pub(crate) struct ShortTransformer;

#[derive(restruct_derive::Struct)]
#[fmt = ">i"]
pub(crate) struct IntegerTransformer;

impl UnsignedCharTransformer {
    pub(crate) fn encode(val: u8) -> <Self as restruct::Struct>::Packed {
        Self::pack((val,))
    }
}

impl UnsignedShortTransformer {
    pub(crate) fn encode(val: u16) -> <Self as restruct::Struct>::Packed {
        Self::pack((val,))
    }
}

impl UnsignedIntegerTransformer {
    pub(crate) fn encode(val: u32) -> <Self as restruct::Struct>::Packed {
        Self::pack((val,))
    }
}

impl SignedCharTransformer {
    pub(crate) fn encode(val: i8) -> <Self as restruct::Struct>::Packed {
        Self::pack((val,))
    }
}

impl ShortTransformer {
    pub(crate) fn encode(val: i16) -> <Self as restruct::Struct>::Packed {
        Self::pack((val,))
    }
}

impl IntegerTransformer {
    pub(crate) fn encode(val: i32) -> <Self as restruct::Struct>::Packed {
        Self::pack((val,))
    }
}
