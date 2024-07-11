//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbFootIkModifierLeg`
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

/// `hkbFootIkModifierLeg`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 160
/// -    vtable: false
/// - signature: `0x9f3e3a04`
/// -   version: 2
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbFootIkModifierLeg<'a> {
    /// # C++ Class Fields Info
    /// -   name:`"originalAnkleTransformMS"`
    /// -   type: `hkQsTransform`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub original_ankle_transform_ms: QsTransform<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"prevAnkleRotLS"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub prev_ankle_rot_ls: Quaternion<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"kneeAxisLS"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    pub knee_axis_ls: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"footEndLS"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    pub foot_end_ls: Vector4<f32>,
    /// # C++ Class Fields Info
    /// -   name:`"ungroundedEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    pub ungrounded_event: SingleClass<HkbEventProperty<'a>>,
    /// # C++ Class Fields Info
    /// -   name:`"footPlantedAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    pub foot_planted_ankle_height_ms: f32,
    /// # C++ Class Fields Info
    /// -   name:`"footRaisedAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    pub foot_raised_ankle_height_ms: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    pub max_ankle_height_ms: f32,
    /// # C++ Class Fields Info
    /// -   name:`"minAnkleHeightMS"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    pub min_ankle_height_ms: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxKneeAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    pub max_knee_angle_degrees: f32,
    /// # C++ Class Fields Info
    /// -   name:`"minKneeAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    pub min_knee_angle_degrees: f32,
    /// # C++ Class Fields Info
    /// -   name:`"verticalError"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    pub vertical_error: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxAnkleAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    pub max_ankle_angle_degrees: f32,
    /// # C++ Class Fields Info
    /// -   name:`"hipIndex"`
    /// -   type: `hkInt16`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE`
    pub hip_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"kneeIndex"`
    /// -   type: `hkInt16`
    /// - offset: 138
    /// -  flags: `FLAGS_NONE`
    pub knee_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"ankleIndex"`
    /// -   type: `hkInt16`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE`
    pub ankle_index: i16,
    /// # C++ Class Fields Info
    /// -   name:`"hitSomething"`
    /// -   type: `hkBool`
    /// - offset: 142
    /// -  flags: `FLAGS_NONE`
    pub hit_something: bool,
    /// # C++ Class Fields Info
    /// -   name:`"isPlantedMS"`
    /// -   type: `hkBool`
    /// - offset: 143
    /// -  flags: `FLAGS_NONE`
    pub is_planted_ms: bool,
    /// # C++ Class Fields Info
    /// -   name:`"isOriginalAnkleTransformMSSet"`
    /// -   type: `hkBool`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    pub is_original_ankle_transform_ms_set: bool,
}

impl Serialize for HkbFootIkModifierLeg<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbFootIkModifierLegVisitor<'_>> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbFootIkModifierLeg<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbFootIkModifierLegVisitor<'de>>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl<'a> From<Vec<HkbFootIkModifierLegVisitor<'a>>> for HkbFootIkModifierLeg<'a> {
    fn from(_values: Vec<HkbFootIkModifierLegVisitor<'a>>) -> Self {
            let mut original_ankle_transform_ms = None;
            let mut prev_ankle_rot_ls = None;
            let mut knee_axis_ls = None;
            let mut foot_end_ls = None;
            let mut ungrounded_event = None;
            let mut foot_planted_ankle_height_ms = None;
            let mut foot_raised_ankle_height_ms = None;
            let mut max_ankle_height_ms = None;
            let mut min_ankle_height_ms = None;
            let mut max_knee_angle_degrees = None;
            let mut min_knee_angle_degrees = None;
            let mut vertical_error = None;
            let mut max_ankle_angle_degrees = None;
            let mut hip_index = None;
            let mut knee_index = None;
            let mut ankle_index = None;
            let mut hit_something = None;
            let mut is_planted_ms = None;
            let mut is_original_ankle_transform_ms_set = None;


        for _value in _values {
            match _value {
                HkbFootIkModifierLegVisitor::OriginalAnkleTransformMs(m) => original_ankle_transform_ms = Some(m),
                HkbFootIkModifierLegVisitor::PrevAnkleRotLs(m) => prev_ankle_rot_ls = Some(m),
                HkbFootIkModifierLegVisitor::KneeAxisLs(m) => knee_axis_ls = Some(m),
                HkbFootIkModifierLegVisitor::FootEndLs(m) => foot_end_ls = Some(m),
                HkbFootIkModifierLegVisitor::UngroundedEvent(m) => ungrounded_event = Some(m),
                HkbFootIkModifierLegVisitor::FootPlantedAnkleHeightMs(m) => foot_planted_ankle_height_ms = Some(m),
                HkbFootIkModifierLegVisitor::FootRaisedAnkleHeightMs(m) => foot_raised_ankle_height_ms = Some(m),
                HkbFootIkModifierLegVisitor::MaxAnkleHeightMs(m) => max_ankle_height_ms = Some(m),
                HkbFootIkModifierLegVisitor::MinAnkleHeightMs(m) => min_ankle_height_ms = Some(m),
                HkbFootIkModifierLegVisitor::MaxKneeAngleDegrees(m) => max_knee_angle_degrees = Some(m),
                HkbFootIkModifierLegVisitor::MinKneeAngleDegrees(m) => min_knee_angle_degrees = Some(m),
                HkbFootIkModifierLegVisitor::VerticalError(m) => vertical_error = Some(m),
                HkbFootIkModifierLegVisitor::MaxAnkleAngleDegrees(m) => max_ankle_angle_degrees = Some(m),
                HkbFootIkModifierLegVisitor::HipIndex(m) => hip_index = Some(m),
                HkbFootIkModifierLegVisitor::KneeIndex(m) => knee_index = Some(m),
                HkbFootIkModifierLegVisitor::AnkleIndex(m) => ankle_index = Some(m),
                HkbFootIkModifierLegVisitor::HitSomething(m) => hit_something = Some(m),
                HkbFootIkModifierLegVisitor::IsPlantedMs(m) => is_planted_ms = Some(m),
                HkbFootIkModifierLegVisitor::IsOriginalAnkleTransformMsSet(m) => is_original_ankle_transform_ms_set = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            original_ankle_transform_ms: original_ankle_transform_ms.unwrap_or_default().into_inner(),
            prev_ankle_rot_ls: prev_ankle_rot_ls.unwrap_or_default().into_inner(),
            knee_axis_ls: knee_axis_ls.unwrap_or_default().into_inner(),
            foot_end_ls: foot_end_ls.unwrap_or_default().into_inner(),
            ungrounded_event: ungrounded_event.unwrap_or_default(),
            foot_planted_ankle_height_ms: foot_planted_ankle_height_ms.unwrap_or_default().into_inner(),
            foot_raised_ankle_height_ms: foot_raised_ankle_height_ms.unwrap_or_default().into_inner(),
            max_ankle_height_ms: max_ankle_height_ms.unwrap_or_default().into_inner(),
            min_ankle_height_ms: min_ankle_height_ms.unwrap_or_default().into_inner(),
            max_knee_angle_degrees: max_knee_angle_degrees.unwrap_or_default().into_inner(),
            min_knee_angle_degrees: min_knee_angle_degrees.unwrap_or_default().into_inner(),
            vertical_error: vertical_error.unwrap_or_default().into_inner(),
            max_ankle_angle_degrees: max_ankle_angle_degrees.unwrap_or_default().into_inner(),
            hip_index: hip_index.unwrap_or_default().into_inner(),
            knee_index: knee_index.unwrap_or_default().into_inner(),
            ankle_index: ankle_index.unwrap_or_default().into_inner(),
            hit_something: hit_something.unwrap_or_default().into_inner(),
            is_planted_ms: is_planted_ms.unwrap_or_default().into_inner(),
            is_original_ankle_transform_ms_set: is_original_ankle_transform_ms_set.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl<'a> From<&HkbFootIkModifierLeg<'a>> for Vec<HkbFootIkModifierLegVisitor<'a>> {
    fn from(data: &HkbFootIkModifierLeg<'a>) -> Self {
        vec![
            HkbFootIkModifierLegVisitor::OriginalAnkleTransformMs(data.original_ankle_transform_ms.clone().into()),
            HkbFootIkModifierLegVisitor::PrevAnkleRotLs(data.prev_ankle_rot_ls.clone().into()),
            HkbFootIkModifierLegVisitor::KneeAxisLs(data.knee_axis_ls.into()),
            HkbFootIkModifierLegVisitor::FootEndLs(data.foot_end_ls.into()),
            HkbFootIkModifierLegVisitor::UngroundedEvent(data.ungrounded_event.clone()),
            HkbFootIkModifierLegVisitor::FootPlantedAnkleHeightMs(data.foot_planted_ankle_height_ms.into()),
            HkbFootIkModifierLegVisitor::FootRaisedAnkleHeightMs(data.foot_raised_ankle_height_ms.into()),
            HkbFootIkModifierLegVisitor::MaxAnkleHeightMs(data.max_ankle_height_ms.into()),
            HkbFootIkModifierLegVisitor::MinAnkleHeightMs(data.min_ankle_height_ms.into()),
            HkbFootIkModifierLegVisitor::MaxKneeAngleDegrees(data.max_knee_angle_degrees.into()),
            HkbFootIkModifierLegVisitor::MinKneeAngleDegrees(data.min_knee_angle_degrees.into()),
            HkbFootIkModifierLegVisitor::VerticalError(data.vertical_error.into()),
            HkbFootIkModifierLegVisitor::MaxAnkleAngleDegrees(data.max_ankle_angle_degrees.into()),
            HkbFootIkModifierLegVisitor::HipIndex(data.hip_index.into()),
            HkbFootIkModifierLegVisitor::KneeIndex(data.knee_index.into()),
            HkbFootIkModifierLegVisitor::AnkleIndex(data.ankle_index.into()),
            HkbFootIkModifierLegVisitor::HitSomething(data.hit_something.into()),
            HkbFootIkModifierLegVisitor::IsPlantedMs(data.is_planted_ms.into()),
            HkbFootIkModifierLegVisitor::IsOriginalAnkleTransformMsSet(data.is_original_ankle_transform_ms_set.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbFootIkModifierLeg<'de> {
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
enum HkbFootIkModifierLegVisitor<'a> {
    /// Visitor fields
    #[serde(rename = "originalAnkleTransformMS")]
    OriginalAnkleTransformMs(Primitive<QsTransform<f32>>),
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
    #[serde(rename = "ungroundedEvent")]
    UngroundedEvent(SingleClass<HkbEventProperty<'a>>),
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
    #[serde(rename = "verticalError")]
    VerticalError(Primitive<f32>),
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
    /// Visitor fields
    #[serde(rename = "hitSomething")]
    HitSomething(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "isPlantedMS")]
    IsPlantedMs(Primitive<bool>),
    /// Visitor fields
    #[serde(rename = "isOriginalAnkleTransformMSSet")]
    IsOriginalAnkleTransformMsSet(Primitive<bool>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkModifierLegVisitor<'de>, "@name",
    ("originalAnkleTransformMS" => OriginalAnkleTransformMs(Primitive<QsTransform<f32>>)),
    ("prevAnkleRotLS" => PrevAnkleRotLs(Primitive<Quaternion<f32>>)),
    ("kneeAxisLS" => KneeAxisLs(Primitive<Vector4<f32>>)),
    ("footEndLS" => FootEndLs(Primitive<Vector4<f32>>)),
    ("ungroundedEvent" => UngroundedEvent(SingleClass<HkbEventProperty<'de>>)),
    ("footPlantedAnkleHeightMS" => FootPlantedAnkleHeightMs(Primitive<f32>)),
    ("footRaisedAnkleHeightMS" => FootRaisedAnkleHeightMs(Primitive<f32>)),
    ("maxAnkleHeightMS" => MaxAnkleHeightMs(Primitive<f32>)),
    ("minAnkleHeightMS" => MinAnkleHeightMs(Primitive<f32>)),
    ("maxKneeAngleDegrees" => MaxKneeAngleDegrees(Primitive<f32>)),
    ("minKneeAngleDegrees" => MinKneeAngleDegrees(Primitive<f32>)),
    ("verticalError" => VerticalError(Primitive<f32>)),
    ("maxAnkleAngleDegrees" => MaxAnkleAngleDegrees(Primitive<f32>)),
    ("hipIndex" => HipIndex(Primitive<i16>)),
    ("kneeIndex" => KneeIndex(Primitive<i16>)),
    ("ankleIndex" => AnkleIndex(Primitive<i16>)),
    ("hitSomething" => HitSomething(Primitive<bool>)),
    ("isPlantedMS" => IsPlantedMs(Primitive<bool>)),
    ("isOriginalAnkleTransformMSSet" => IsOriginalAnkleTransformMsSet(Primitive<bool>)),
}
