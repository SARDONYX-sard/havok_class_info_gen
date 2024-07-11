//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbFootIkDriverInfoLeg`
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

/// `hkbFootIkDriverInfoLeg`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 96
/// -    vtable: false
/// - signature: `0x224b18d1`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbFootIkDriverInfoLeg {
    /// # C++ Class Fields Info
    /// -   name:`"prevAnkleRotLS"`
    /// -   type: `hkQuaternion`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub prev_ankle_rot_ls: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"kneeAxisLS"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub knee_axis_ls: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"footEndLS"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub foot_end_ls: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"footPlantedAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    pub foot_planted_ankle_height_ms: f32,
    /// # C++ Class Fields Info
    /// -   name:`"footRaisedAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    pub foot_raised_ankle_height_ms: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    pub max_ankle_height_ms: f32,
    /// # C++ Class Fields Info
    /// -   name:`"minAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    pub min_ankle_height_ms: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxKneeAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub max_knee_angle_degrees: f32,
    /// # C++ Class Fields Info
    /// -   name:`"minKneeAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    pub min_knee_angle_degrees: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxAnkleAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    pub max_ankle_angle_degrees: f32,
    /// # C++ Class Fields Info
    /// -   name:`"hipIndex"`
    /// -   type: `hkInt16`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    pub hip_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"kneeIndex"`
    /// -   type: `hkInt16`
    /// - offset: 78
    /// -  flags: `FLAGS_NONE`
    pub knee_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"ankleIndex"`
    /// -   type: `hkInt16`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub ankle_index: i16,
}

impl Serialize for HkbFootIkDriverInfoLeg {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbFootIkDriverInfoLegVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbFootIkDriverInfoLeg {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbFootIkDriverInfoLegVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbFootIkDriverInfoLegVisitor>> for HkbFootIkDriverInfoLeg {
    fn from(_values: Vec<HkbFootIkDriverInfoLegVisitor>) -> Self {
            let mut prev_ankle_rot_ls = None;
            let mut knee_axis_ls = None;
            let mut foot_end_ls = None;
            let mut foot_planted_ankle_height_ms = None;
            let mut foot_raised_ankle_height_ms = None;
            let mut max_ankle_height_ms = None;
            let mut min_ankle_height_ms = None;
            let mut max_knee_angle_degrees = None;
            let mut min_knee_angle_degrees = None;
            let mut max_ankle_angle_degrees = None;
            let mut hip_index = None;
            let mut knee_index = None;
            let mut ankle_index = None;


        for _value in _values {
            match _value {
                HkbFootIkDriverInfoLegVisitor::PrevAnkleRotLs(m) => prev_ankle_rot_ls = Some(m),
                HkbFootIkDriverInfoLegVisitor::KneeAxisLs(m) => knee_axis_ls = Some(m),
                HkbFootIkDriverInfoLegVisitor::FootEndLs(m) => foot_end_ls = Some(m),
                HkbFootIkDriverInfoLegVisitor::FootPlantedAnkleHeightMs(m) => foot_planted_ankle_height_ms = Some(m),
                HkbFootIkDriverInfoLegVisitor::FootRaisedAnkleHeightMs(m) => foot_raised_ankle_height_ms = Some(m),
                HkbFootIkDriverInfoLegVisitor::MaxAnkleHeightMs(m) => max_ankle_height_ms = Some(m),
                HkbFootIkDriverInfoLegVisitor::MinAnkleHeightMs(m) => min_ankle_height_ms = Some(m),
                HkbFootIkDriverInfoLegVisitor::MaxKneeAngleDegrees(m) => max_knee_angle_degrees = Some(m),
                HkbFootIkDriverInfoLegVisitor::MinKneeAngleDegrees(m) => min_knee_angle_degrees = Some(m),
                HkbFootIkDriverInfoLegVisitor::MaxAnkleAngleDegrees(m) => max_ankle_angle_degrees = Some(m),
                HkbFootIkDriverInfoLegVisitor::HipIndex(m) => hip_index = Some(m),
                HkbFootIkDriverInfoLegVisitor::KneeIndex(m) => knee_index = Some(m),
                HkbFootIkDriverInfoLegVisitor::AnkleIndex(m) => ankle_index = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            prev_ankle_rot_ls: prev_ankle_rot_ls.unwrap_or_default().into_inner(),
            knee_axis_ls: knee_axis_ls.unwrap_or_default().into_inner(),
            foot_end_ls: foot_end_ls.unwrap_or_default().into_inner(),
            foot_planted_ankle_height_ms: foot_planted_ankle_height_ms.unwrap_or_default().into_inner(),
            foot_raised_ankle_height_ms: foot_raised_ankle_height_ms.unwrap_or_default().into_inner(),
            max_ankle_height_ms: max_ankle_height_ms.unwrap_or_default().into_inner(),
            min_ankle_height_ms: min_ankle_height_ms.unwrap_or_default().into_inner(),
            max_knee_angle_degrees: max_knee_angle_degrees.unwrap_or_default().into_inner(),
            min_knee_angle_degrees: min_knee_angle_degrees.unwrap_or_default().into_inner(),
            max_ankle_angle_degrees: max_ankle_angle_degrees.unwrap_or_default().into_inner(),
            hip_index: hip_index.unwrap_or_default().into_inner(),
            knee_index: knee_index.unwrap_or_default().into_inner(),
            ankle_index: ankle_index.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbFootIkDriverInfoLeg> for Vec<HkbFootIkDriverInfoLegVisitor> {
    fn from(data: &HkbFootIkDriverInfoLeg) -> Self {
        vec![
            HkbFootIkDriverInfoLegVisitor::PrevAnkleRotLs(data.prev_ankle_rot_ls.clone().into()),
            HkbFootIkDriverInfoLegVisitor::KneeAxisLs(data.knee_axis_ls.into()),
            HkbFootIkDriverInfoLegVisitor::FootEndLs(data.foot_end_ls.into()),
            HkbFootIkDriverInfoLegVisitor::FootPlantedAnkleHeightMs(data.foot_planted_ankle_height_ms.into()),
            HkbFootIkDriverInfoLegVisitor::FootRaisedAnkleHeightMs(data.foot_raised_ankle_height_ms.into()),
            HkbFootIkDriverInfoLegVisitor::MaxAnkleHeightMs(data.max_ankle_height_ms.into()),
            HkbFootIkDriverInfoLegVisitor::MinAnkleHeightMs(data.min_ankle_height_ms.into()),
            HkbFootIkDriverInfoLegVisitor::MaxKneeAngleDegrees(data.max_knee_angle_degrees.into()),
            HkbFootIkDriverInfoLegVisitor::MinKneeAngleDegrees(data.min_knee_angle_degrees.into()),
            HkbFootIkDriverInfoLegVisitor::MaxAnkleAngleDegrees(data.max_ankle_angle_degrees.into()),
            HkbFootIkDriverInfoLegVisitor::HipIndex(data.hip_index.into()),
            HkbFootIkDriverInfoLegVisitor::KneeIndex(data.knee_index.into()),
            HkbFootIkDriverInfoLegVisitor::AnkleIndex(data.ankle_index.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbFootIkDriverInfoLeg {
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
enum HkbFootIkDriverInfoLegVisitor {
    /// Visitor fields
    #[serde(rename = "prevAnkleRotLS", skip_serializing)]
    PrevAnkleRotLs(Primitive<Quaternion<f32>>),
    /// Visitor fields
    #[serde(rename = "kneeAxisLS")]
    KneeAxisLs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "footEndLS")]
    FootEndLs(Primitive<Vector4<f32>>),
    /// Visitor fields
    #[serde(rename = "footPlantedAnkleHeightMS")]
    FootPlantedAnkleHeightMs(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "footRaisedAnkleHeightMS")]
    FootRaisedAnkleHeightMs(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxAnkleHeightMS")]
    MaxAnkleHeightMs(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "minAnkleHeightMS")]
    MinAnkleHeightMs(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxKneeAngleDegrees")]
    MaxKneeAngleDegrees(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "minKneeAngleDegrees")]
    MinKneeAngleDegrees(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxAnkleAngleDegrees")]
    MaxAnkleAngleDegrees(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "hipIndex")]
    HipIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "kneeIndex")]
    KneeIndex(Primitive<i16>),
    /// Visitor fields
    #[serde(rename = "ankleIndex")]
    AnkleIndex(Primitive<i16>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkDriverInfoLegVisitor, "@name",
    ("prevAnkleRotLS" => PrevAnkleRotLs(Primitive<Quaternion<f32>>)),
    ("kneeAxisLS" => KneeAxisLs(Primitive<Vector4<f32>>)),
    ("footEndLS" => FootEndLs(Primitive<Vector4<f32>>)),
    ("footPlantedAnkleHeightMS" => FootPlantedAnkleHeightMs(Primitive<f32>)),
    ("footRaisedAnkleHeightMS" => FootRaisedAnkleHeightMs(Primitive<f32>)),
    ("maxAnkleHeightMS" => MaxAnkleHeightMs(Primitive<f32>)),
    ("minAnkleHeightMS" => MinAnkleHeightMs(Primitive<f32>)),
    ("maxKneeAngleDegrees" => MaxKneeAngleDegrees(Primitive<f32>)),
    ("minKneeAngleDegrees" => MinKneeAngleDegrees(Primitive<f32>)),
    ("maxAnkleAngleDegrees" => MaxAnkleAngleDegrees(Primitive<f32>)),
    ("hipIndex" => HipIndex(Primitive<i16>)),
    ("kneeIndex" => KneeIndex(Primitive<i16>)),
    ("ankleIndex" => AnkleIndex(Primitive<i16>)),
}
