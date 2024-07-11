//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbClipGeneratorEcho`
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

/// `hkbClipGeneratorEcho`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 16
/// -    vtable: false
/// - signature: `0x750edf40`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbClipGeneratorEcho {
    /// # C++ Class Fields Info
    /// -   name:`"offsetLocalTime"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|ALIGN16`
    pub offset_local_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"weight"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub weight: f32,
    /// # C++ Class Fields Info
    /// -   name:`"dwdt"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub dwdt: f32,
}

impl Serialize for HkbClipGeneratorEcho {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbClipGeneratorEchoVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbClipGeneratorEcho {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbClipGeneratorEchoVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbClipGeneratorEchoVisitor>> for HkbClipGeneratorEcho {
    fn from(_values: Vec<HkbClipGeneratorEchoVisitor>) -> Self {
            let mut offset_local_time = None;
            let mut weight = None;
            let mut dwdt = None;


        for _value in _values {
            match _value {
                HkbClipGeneratorEchoVisitor::OffsetLocalTime(m) => offset_local_time = Some(m),
                HkbClipGeneratorEchoVisitor::Weight(m) => weight = Some(m),
                HkbClipGeneratorEchoVisitor::Dwdt(m) => dwdt = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            offset_local_time: offset_local_time.unwrap_or_default().into_inner(),
            weight: weight.unwrap_or_default().into_inner(),
            dwdt: dwdt.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbClipGeneratorEcho> for Vec<HkbClipGeneratorEchoVisitor> {
    fn from(data: &HkbClipGeneratorEcho) -> Self {
        vec![
            HkbClipGeneratorEchoVisitor::OffsetLocalTime(data.offset_local_time.into()),
            HkbClipGeneratorEchoVisitor::Weight(data.weight.into()),
            HkbClipGeneratorEchoVisitor::Dwdt(data.dwdt.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbClipGeneratorEcho {
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
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "@name")]
enum HkbClipGeneratorEchoVisitor {
    /// Visitor fields
    #[serde(rename = "offsetLocalTime")]
    OffsetLocalTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "weight")]
    Weight(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "dwdt")]
    Dwdt(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbClipGeneratorEchoVisitor, "@name",
    ("offsetLocalTime" => OffsetLocalTime(Primitive<f32>)),
    ("weight" => Weight(Primitive<f32>)),
    ("dwdt" => Dwdt(Primitive<f32>)),
}
