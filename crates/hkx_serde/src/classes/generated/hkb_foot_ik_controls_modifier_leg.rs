//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbFootIkControlsModifierLeg`
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

/// `hkbFootIkControlsModifierLeg`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0x9e17091a`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbFootIkControlsModifierLeg<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"groundPosition"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub ground_position: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"ungroundedEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub ungrounded_event: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"verticalError"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub vertical_error: f32,
    /// # C++ Class Fields Info
    /// -   name:`"hitSomething"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub hit_something: bool,
    /// # C++ Class Fields Info
    /// -   name:`"isPlantedMS"`
    /// -   type: `hkBool`
    /// - offset: 29
    /// -  flags: `FLAGS_NONE`
    pub is_planted_ms: bool,
}

impl Serialize for HkbFootIkControlsModifierLeg<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbFootIkControlsModifierLegVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbFootIkControlsModifierLeg<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbFootIkControlsModifierLegVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbFootIkControlsModifierLegVisitor<'a>>> for HkbFootIkControlsModifierLeg<'a> {
    fn from(_values: Vec<HkbFootIkControlsModifierLegVisitor<'a>>) -> Self {
            let mut ground_position = None;
            let mut ungrounded_event = None;
            let mut vertical_error = None;
            let mut hit_something = None;
            let mut is_planted_ms = None;


        for _value in _values {
            match _value {
                HkbFootIkControlsModifierLegVisitor::GroundPosition(m) => ground_position = Some(m),
                HkbFootIkControlsModifierLegVisitor::UngroundedEvent(m) => ungrounded_event = Some(m),
                HkbFootIkControlsModifierLegVisitor::VerticalError(m) => vertical_error = Some(m),
                HkbFootIkControlsModifierLegVisitor::HitSomething(m) => hit_something = Some(m),
                HkbFootIkControlsModifierLegVisitor::IsPlantedMs(m) => is_planted_ms = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            ground_position: ground_position.unwrap_or_default().into_inner(),
            ungrounded_event: ungrounded_event.unwrap_or_default(),
            vertical_error: vertical_error.unwrap_or_default().into_inner(),
            hit_something: hit_something.unwrap_or_default().into_inner(),
            is_planted_ms: is_planted_ms.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbFootIkControlsModifierLeg<'a>> for Vec<HkbFootIkControlsModifierLegVisitor<'a>> {
    fn from(data: &HkbFootIkControlsModifierLeg<'a>) -> Self {
        vec![
            HkbFootIkControlsModifierLegVisitor::GroundPosition(data.ground_position.into()),
            HkbFootIkControlsModifierLegVisitor::UngroundedEvent(data.ungrounded_event.clone()),
            HkbFootIkControlsModifierLegVisitor::VerticalError(data.vertical_error.into()),
            HkbFootIkControlsModifierLegVisitor::HitSomething(data.hit_something.into()),
            HkbFootIkControlsModifierLegVisitor::IsPlantedMs(data.is_planted_ms.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbFootIkControlsModifierLeg<'de> {
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
enum HkbFootIkControlsModifierLegVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "groundPosition")]
    GroundPosition(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "ungroundedEvent")]
    UngroundedEvent(SingleClass<HkbEventProperty<'a>>),
    /// Visitor fields
    #[serde(rename = "verticalError")]
    VerticalError(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "hitSomething")]
    HitSomething(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "isPlantedMS")]
    IsPlantedMs(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkControlsModifierLegVisitor<'de>, "@name",
    ("groundPosition" => GroundPosition(Primitive<Vector4<f32>>)),
    ("ungroundedEvent" => UngroundedEvent(SingleClass<HkbEventProperty<'de>>)),
    ("verticalError" => VerticalError(Primitive<f32>)),
    ("hitSomething" => HitSomething(Primitive<bool>)),
    ("isPlantedMS" => IsPlantedMs(Primitive<bool>)),
}
