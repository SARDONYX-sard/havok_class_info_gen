//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkpWeldingUtility`
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
#![allow(
  clippy::clone_on_copy,
  clippy::unit_arg
)]

#[allow(unused)]
use super::*;
#[allow(unused)]
use crate::bytes::*; // For hkx binary read/write
#[allow(unused)]
use crate::error::{HkxError, Result};
use crate::havok_types::*;

/// `hkpWeldingUtility`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 1
/// -    vtable: false
/// - signature: `0xb2b41feb`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HkpWeldingUtility {
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkpWeldingUtility {
    fn from_bytes<B>(
        _bytes: &'bytes [u8],
        _de: &mut PackFileDeserializer,
    ) -> Result<Self>
    where
        B: ByteOrder,
        Self: Sized + 'de
    {
        todo!()
    }
}


/// # Why use Visitor pattern?
/// Since the C++ field must be deserialized from the `name` attribute name of the `hkparam` in the XML,
/// this is accomplished by having the Visitor process the internally tagged enum and convert it.
/// Leakage of field items may occur if Vec<enum> is left as it is.
///
/// struct -> (De)serialize by visitor -> struct
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "@name")]
enum HkpWeldingUtilityVisitor {
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum WeldingType {
    #[serde(rename = "WELDING_TYPE_ANTICLOCKWISE")]
    #[default]
    WeldingTypeAnticlockwise = 0,
    #[serde(rename = "WELDING_TYPE_CLOCKWISE")]
    WeldingTypeClockwise = 4,
    #[serde(rename = "WELDING_TYPE_TWO_SIDED")]
    WeldingTypeTwoSided = 5,
    #[serde(rename = "WELDING_TYPE_NONE")]
    WeldingTypeNone = 6,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum SectorType {
    #[serde(rename = "ACCEPT_0")]
    #[default]
    Accept0 = 1,
    #[serde(rename = "SNAP_0")]
    Snap0 = 0,
    #[serde(rename = "REJECT")]
    Reject = 2,
    #[serde(rename = "SNAP_1")]
    Snap1 = 4,
    #[serde(rename = "ACCEPT_1")]
    Accept1 = 3,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum NumAngles {
    #[serde(rename = "NUM_ANGLES")]
    #[default]
    NumAngles = 31,
}
