//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpAgent1nSector`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#[allow(unused)]
use super::*;
use crate::havok_types::*;

/// `hkpAgent1nSector`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 512
/// -    vtable: false
/// - signature: `0x626e55a`
/// -   version: 0
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpAgent1NSector {
    /// # C++ Class Fields Info
    /// -   name:`"bytesAllocated"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bytesAllocated")]
    BytesAllocated(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"pad0"`
    /// -   type: `hkUint32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pad0")]
    Pad0(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"pad1"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pad1")]
    Pad1(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"pad2"`
    /// -   type: `hkUint32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pad2")]
    Pad2(Primitive<u32>),
    /// # C++ Class Fields Info
    /// -   name:`"data"`
    /// -   type: `hkUint8[496]`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(CStyleArray<[u8; 496]>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkpAgent1NSector, "@name",
    ("bytesAllocated" => BytesAllocated(Primitive<u32>)),
    ("pad0" => Pad0(Primitive<u32>)),
    ("pad1" => Pad1(Primitive<u32>)),
    ("pad2" => Pad2(Primitive<u32>)),
    ("data" => Data(CStyleArray<[u8; 496]>)),
}
