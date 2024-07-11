//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbSenseHandleModifierRange`
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

/// `hkbSenseHandleModifierRange`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 20
/// -    vtable: false
/// - signature: `0xfb56b692`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbSenseHandleModifierRange<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"event"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub event: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"minDistance"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub min_distance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxDistance"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub max_distance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"ignoreHandle"`
    /// -   type: `hkBool`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub ignore_handle: bool,
}

impl Serialize for HkbSenseHandleModifierRange<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbSenseHandleModifierRangeVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbSenseHandleModifierRange<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbSenseHandleModifierRangeVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbSenseHandleModifierRangeVisitor<'a>>> for HkbSenseHandleModifierRange<'a> {
    fn from(_values: Vec<HkbSenseHandleModifierRangeVisitor<'a>>) -> Self {
            let mut event = None;
            let mut min_distance = None;
            let mut max_distance = None;
            let mut ignore_handle = None;


        for _value in _values {
            match _value {
                HkbSenseHandleModifierRangeVisitor::Event(m) => event = Some(m),
                HkbSenseHandleModifierRangeVisitor::MinDistance(m) => min_distance = Some(m),
                HkbSenseHandleModifierRangeVisitor::MaxDistance(m) => max_distance = Some(m),
                HkbSenseHandleModifierRangeVisitor::IgnoreHandle(m) => ignore_handle = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            event: event.unwrap_or_default(),
            min_distance: min_distance.unwrap_or_default().into_inner(),
            max_distance: max_distance.unwrap_or_default().into_inner(),
            ignore_handle: ignore_handle.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbSenseHandleModifierRange<'a>> for Vec<HkbSenseHandleModifierRangeVisitor<'a>> {
    fn from(data: &HkbSenseHandleModifierRange<'a>) -> Self {
        vec![
            HkbSenseHandleModifierRangeVisitor::Event(data.event.clone()),
            HkbSenseHandleModifierRangeVisitor::MinDistance(data.min_distance.into()),
            HkbSenseHandleModifierRangeVisitor::MaxDistance(data.max_distance.into()),
            HkbSenseHandleModifierRangeVisitor::IgnoreHandle(data.ignore_handle.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbSenseHandleModifierRange<'de> {
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
enum HkbSenseHandleModifierRangeVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "event")]
    Event(SingleClass<HkbEventProperty<'a>>),
    /// Visitor fields
    #[serde(rename = "minDistance")]
    MinDistance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxDistance")]
    MaxDistance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "ignoreHandle")]
    IgnoreHandle(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbSenseHandleModifierRangeVisitor<'de>, "@name",
    ("event" => Event(SingleClass<HkbEventProperty<'de>>)),
    ("minDistance" => MinDistance(Primitive<f32>)),
    ("maxDistance" => MaxDistance(Primitive<f32>)),
    ("ignoreHandle" => IgnoreHandle(Primitive<bool>)),
}
