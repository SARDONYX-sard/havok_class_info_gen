//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbCharacterControllerControlData`
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

/// `hkbCharacterControllerControlData`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 32
/// -    vtable: false
/// - signature: `0x5b6c03d9`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbCharacterControllerControlData {
    /// # C++ Class Fields Info
    /// -   name:`"desiredVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub desired_velocity: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"verticalGain"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub vertical_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"horizontalCatchUpGain"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub horizontal_catch_up_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxVerticalSeparation"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub max_vertical_separation: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxHorizontalSeparation"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub max_horizontal_separation: f32,
}

impl Serialize for HkbCharacterControllerControlData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbCharacterControllerControlDataVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbCharacterControllerControlData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbCharacterControllerControlDataVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbCharacterControllerControlDataVisitor>> for HkbCharacterControllerControlData {
    fn from(_values: Vec<HkbCharacterControllerControlDataVisitor>) -> Self {
            let mut desired_velocity = None;
            let mut vertical_gain = None;
            let mut horizontal_catch_up_gain = None;
            let mut max_vertical_separation = None;
            let mut max_horizontal_separation = None;


        for _value in _values {
            match _value {
                HkbCharacterControllerControlDataVisitor::DesiredVelocity(m) => desired_velocity = Some(m),
                HkbCharacterControllerControlDataVisitor::VerticalGain(m) => vertical_gain = Some(m),
                HkbCharacterControllerControlDataVisitor::HorizontalCatchUpGain(m) => horizontal_catch_up_gain = Some(m),
                HkbCharacterControllerControlDataVisitor::MaxVerticalSeparation(m) => max_vertical_separation = Some(m),
                HkbCharacterControllerControlDataVisitor::MaxHorizontalSeparation(m) => max_horizontal_separation = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            desired_velocity: desired_velocity.unwrap_or_default().into_inner(),
            vertical_gain: vertical_gain.unwrap_or_default().into_inner(),
            horizontal_catch_up_gain: horizontal_catch_up_gain.unwrap_or_default().into_inner(),
            max_vertical_separation: max_vertical_separation.unwrap_or_default().into_inner(),
            max_horizontal_separation: max_horizontal_separation.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbCharacterControllerControlData> for Vec<HkbCharacterControllerControlDataVisitor> {
    fn from(data: &HkbCharacterControllerControlData) -> Self {
        vec![
            HkbCharacterControllerControlDataVisitor::DesiredVelocity(data.desired_velocity.into()),
            HkbCharacterControllerControlDataVisitor::VerticalGain(data.vertical_gain.into()),
            HkbCharacterControllerControlDataVisitor::HorizontalCatchUpGain(data.horizontal_catch_up_gain.into()),
            HkbCharacterControllerControlDataVisitor::MaxVerticalSeparation(data.max_vertical_separation.into()),
            HkbCharacterControllerControlDataVisitor::MaxHorizontalSeparation(data.max_horizontal_separation.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbCharacterControllerControlData {
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
enum HkbCharacterControllerControlDataVisitor {
    /// Visitor fields
    #[serde(rename = "desiredVelocity")]
    DesiredVelocity(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "verticalGain")]
    VerticalGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "horizontalCatchUpGain")]
    HorizontalCatchUpGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxVerticalSeparation")]
    MaxVerticalSeparation(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxHorizontalSeparation")]
    MaxHorizontalSeparation(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterControllerControlDataVisitor, "@name",
    ("desiredVelocity" => DesiredVelocity(Primitive<Vector4<f32>>)),
    ("verticalGain" => VerticalGain(Primitive<f32>)),
    ("horizontalCatchUpGain" => HorizontalCatchUpGain(Primitive<f32>)),
    ("maxVerticalSeparation" => MaxVerticalSeparation(Primitive<f32>)),
    ("maxHorizontalSeparation" => MaxHorizontalSeparation(Primitive<f32>)),
}
