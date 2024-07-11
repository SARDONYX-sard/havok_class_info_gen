//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbHandIkModifierHand`
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

/// `hkbHandIkModifierHand`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: false
/// - signature: `0x14dfe1dd`
/// -   version: 3
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbHandIkModifierHand<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"elbowAxisLS"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub elbow_axis_ls: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"backHandNormalLS"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub back_hand_normal_ls: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"handOffsetLS"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub hand_offset_ls: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"handOrienationOffsetLS"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub hand_orienation_offset_ls: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"maxElbowAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub max_elbow_angle_degrees: f32,
    /// # C++ Class Fields Info
    /// -   name:`"minElbowAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub min_elbow_angle_degrees: f32,
    /// # C++ Class Fields Info
    /// -   name:`"shoulderIndex"`
    /// -   type: `hkInt16`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub shoulder_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"shoulderSiblingIndex"`
    /// -   type: `hkInt16`
    /// - offset: 74
    /// -  flags: `FLAGS_NONE`
    pub shoulder_sibling_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"elbowIndex"`
    /// -   type: `hkInt16`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub elbow_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"elbowSiblingIndex"`
    /// -   type: `hkInt16`
    /// - offset: 78
    /// -  flags: `FLAGS_NONE`
    pub elbow_sibling_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"wristIndex"`
    /// -   type: `hkInt16`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub wrist_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"enforceEndPosition"`
    /// -   type: `hkBool`
    /// - offset: 82
    /// -  flags: `FLAGS_NONE`
    pub enforce_end_position: bool,
    /// # C++ Class Fields Info
    /// -   name:`"enforceEndRotation"`
    /// -   type: `hkBool`
    /// - offset: 83
    /// -  flags: `FLAGS_NONE`
    pub enforce_end_rotation: bool,
    /// # C++ Class Fields Info
    /// -   name:`"localFrameName"`
    /// -   type: `hkStringPtr`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    pub local_frame_name: Cow<'a, str>,
}

impl Serialize for HkbHandIkModifierHand<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbHandIkModifierHandVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbHandIkModifierHand<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbHandIkModifierHandVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbHandIkModifierHandVisitor<'a>>> for HkbHandIkModifierHand<'a> {
    fn from(_values: Vec<HkbHandIkModifierHandVisitor<'a>>) -> Self {
            let mut elbow_axis_ls = None;
            let mut back_hand_normal_ls = None;
            let mut hand_offset_ls = None;
            let mut hand_orienation_offset_ls = None;
            let mut max_elbow_angle_degrees = None;
            let mut min_elbow_angle_degrees = None;
            let mut shoulder_index = None;
            let mut shoulder_sibling_index = None;
            let mut elbow_index = None;
            let mut elbow_sibling_index = None;
            let mut wrist_index = None;
            let mut enforce_end_position = None;
            let mut enforce_end_rotation = None;
            let mut local_frame_name = None;


        for _value in _values {
            match _value {
                HkbHandIkModifierHandVisitor::ElbowAxisLs(m) => elbow_axis_ls = Some(m),
                HkbHandIkModifierHandVisitor::BackHandNormalLs(m) => back_hand_normal_ls = Some(m),
                HkbHandIkModifierHandVisitor::HandOffsetLs(m) => hand_offset_ls = Some(m),
                HkbHandIkModifierHandVisitor::HandOrienationOffsetLs(m) => hand_orienation_offset_ls = Some(m),
                HkbHandIkModifierHandVisitor::MaxElbowAngleDegrees(m) => max_elbow_angle_degrees = Some(m),
                HkbHandIkModifierHandVisitor::MinElbowAngleDegrees(m) => min_elbow_angle_degrees = Some(m),
                HkbHandIkModifierHandVisitor::ShoulderIndex(m) => shoulder_index = Some(m),
                HkbHandIkModifierHandVisitor::ShoulderSiblingIndex(m) => shoulder_sibling_index = Some(m),
                HkbHandIkModifierHandVisitor::ElbowIndex(m) => elbow_index = Some(m),
                HkbHandIkModifierHandVisitor::ElbowSiblingIndex(m) => elbow_sibling_index = Some(m),
                HkbHandIkModifierHandVisitor::WristIndex(m) => wrist_index = Some(m),
                HkbHandIkModifierHandVisitor::EnforceEndPosition(m) => enforce_end_position = Some(m),
                HkbHandIkModifierHandVisitor::EnforceEndRotation(m) => enforce_end_rotation = Some(m),
                HkbHandIkModifierHandVisitor::LocalFrameName(m) => local_frame_name = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            elbow_axis_ls: elbow_axis_ls.unwrap_or_default().into_inner(),
            back_hand_normal_ls: back_hand_normal_ls.unwrap_or_default().into_inner(),
            hand_offset_ls: hand_offset_ls.unwrap_or_default().into_inner(),
            hand_orienation_offset_ls: hand_orienation_offset_ls.unwrap_or_default().into_inner(),
            max_elbow_angle_degrees: max_elbow_angle_degrees.unwrap_or_default().into_inner(),
            min_elbow_angle_degrees: min_elbow_angle_degrees.unwrap_or_default().into_inner(),
            shoulder_index: shoulder_index.unwrap_or_default().into_inner(),
            shoulder_sibling_index: shoulder_sibling_index.unwrap_or_default().into_inner(),
            elbow_index: elbow_index.unwrap_or_default().into_inner(),
            elbow_sibling_index: elbow_sibling_index.unwrap_or_default().into_inner(),
            wrist_index: wrist_index.unwrap_or_default().into_inner(),
            enforce_end_position: enforce_end_position.unwrap_or_default().into_inner(),
            enforce_end_rotation: enforce_end_rotation.unwrap_or_default().into_inner(),
            local_frame_name: local_frame_name.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbHandIkModifierHand<'a>> for Vec<HkbHandIkModifierHandVisitor<'a>> {
    fn from(data: &HkbHandIkModifierHand<'a>) -> Self {
        vec![
            HkbHandIkModifierHandVisitor::ElbowAxisLs(data.elbow_axis_ls.into()),
            HkbHandIkModifierHandVisitor::BackHandNormalLs(data.back_hand_normal_ls.into()),
            HkbHandIkModifierHandVisitor::HandOffsetLs(data.hand_offset_ls.into()),
            HkbHandIkModifierHandVisitor::HandOrienationOffsetLs(data.hand_orienation_offset_ls.clone().into()),
            HkbHandIkModifierHandVisitor::MaxElbowAngleDegrees(data.max_elbow_angle_degrees.into()),
            HkbHandIkModifierHandVisitor::MinElbowAngleDegrees(data.min_elbow_angle_degrees.into()),
            HkbHandIkModifierHandVisitor::ShoulderIndex(data.shoulder_index.into()),
            HkbHandIkModifierHandVisitor::ShoulderSiblingIndex(data.shoulder_sibling_index.into()),
            HkbHandIkModifierHandVisitor::ElbowIndex(data.elbow_index.into()),
            HkbHandIkModifierHandVisitor::ElbowSiblingIndex(data.elbow_sibling_index.into()),
            HkbHandIkModifierHandVisitor::WristIndex(data.wrist_index.into()),
            HkbHandIkModifierHandVisitor::EnforceEndPosition(data.enforce_end_position.into()),
            HkbHandIkModifierHandVisitor::EnforceEndRotation(data.enforce_end_rotation.into()),
            HkbHandIkModifierHandVisitor::LocalFrameName(data.local_frame_name.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbHandIkModifierHand<'de> {
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
enum HkbHandIkModifierHandVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "elbowAxisLS")]
    ElbowAxisLs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "backHandNormalLS")]
    BackHandNormalLs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "handOffsetLS")]
    HandOffsetLs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "handOrienationOffsetLS")]
    HandOrienationOffsetLs(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "maxElbowAngleDegrees")]
    MaxElbowAngleDegrees(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "minElbowAngleDegrees")]
    MinElbowAngleDegrees(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "shoulderIndex")]
    ShoulderIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "shoulderSiblingIndex")]
    ShoulderSiblingIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "elbowIndex")]
    ElbowIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "elbowSiblingIndex")]
    ElbowSiblingIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "wristIndex")]
    WristIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "enforceEndPosition")]
    EnforceEndPosition(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "enforceEndRotation")]
    EnforceEndRotation(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "localFrameName")]
    LocalFrameName(Primitive<Cow<'a, str>>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandIkModifierHandVisitor<'de>, "@name",
    ("elbowAxisLS" => ElbowAxisLs(Primitive<Vector4<f32>>)),
    ("backHandNormalLS" => BackHandNormalLs(Primitive<Vector4<f32>>)),
    ("handOffsetLS" => HandOffsetLs(Primitive<Vector4<f32>>)),
    ("handOrienationOffsetLS" => HandOrienationOffsetLs(Primitive<Quaternion<f32>>)),
    ("maxElbowAngleDegrees" => MaxElbowAngleDegrees(Primitive<f32>)),
    ("minElbowAngleDegrees" => MinElbowAngleDegrees(Primitive<f32>)),
    ("shoulderIndex" => ShoulderIndex(Primitive<i16>)),
    ("shoulderSiblingIndex" => ShoulderSiblingIndex(Primitive<i16>)),
    ("elbowIndex" => ElbowIndex(Primitive<i16>)),
    ("elbowSiblingIndex" => ElbowSiblingIndex(Primitive<i16>)),
    ("wristIndex" => WristIndex(Primitive<i16>)),
    ("enforceEndPosition" => EnforceEndPosition(Primitive<bool>)),
    ("enforceEndRotation" => EnforceEndRotation(Primitive<bool>)),
    ("localFrameName" => LocalFrameName(Primitive<Cow<'de, str>>)),
}
