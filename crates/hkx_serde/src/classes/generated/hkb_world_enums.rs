//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbWorldEnums`
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

/// `hkbWorldEnums`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 1
/// -    vtable: false
/// - signature: `0x25640b46`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HkbWorldEnums {
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbWorldEnums {
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
enum HkbWorldEnumsVisitor {
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum SimulationState {
    #[serde(rename = "SIMULATION_STATE_PLAY")]
    #[default]
    SimulationStatePlay = 0,
    #[serde(rename = "SIMULATION_STATE_PAUSE")]
    SimulationStatePause = 1,
    #[serde(rename = "SIMULATION_STATE_STEP")]
    SimulationStateStep = 2,
    #[serde(rename = "SIMULATION_STATE_STOP")]
    SimulationStateStop = 3,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum AccumulateMotionState {
    #[serde(rename = "ACCUMULATE_MOTION")]
    #[default]
    AccumulateMotion = 0,
    #[serde(rename = "DO_NOT_ACCUMULATE_MOTION")]
    DoNotAccumulateMotion = 1,
}
