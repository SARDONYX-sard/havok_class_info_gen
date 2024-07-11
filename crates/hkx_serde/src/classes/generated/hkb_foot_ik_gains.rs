//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbFootIkGains`
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

/// `hkbFootIkGains`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 48
/// -    vtable: false
/// - signature: `0xa681b7f0`
/// -   version: 0
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbFootIkGains {
    /// # C++ Class Fields Info
    /// -   name:`"onOffGain"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    pub on_off_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"groundAscendingGain"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    pub ground_ascending_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"groundDescendingGain"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub ground_descending_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"footPlantedGain"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub foot_planted_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"footRaisedGain"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub foot_raised_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"footUnlockGain"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub foot_unlock_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"worldFromModelFeedbackGain"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub world_from_model_feedback_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"errorUpDownBias"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub error_up_down_bias: f32,
    /// # C++ Class Fields Info
    /// -   name:`"alignWorldFromModelGain"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub align_world_from_model_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"hipOrientationGain"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub hip_orientation_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxKneeAngleDifference"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    pub max_knee_angle_difference: f32,
    /// # C++ Class Fields Info
    /// -   name:`"ankleOrientationGain"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    pub ankle_orientation_gain: f32,
}

impl Serialize for HkbFootIkGains {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbFootIkGainsVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbFootIkGains {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbFootIkGainsVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbFootIkGainsVisitor>> for HkbFootIkGains {
    fn from(_values: Vec<HkbFootIkGainsVisitor>) -> Self {
            let mut on_off_gain = None;
            let mut ground_ascending_gain = None;
            let mut ground_descending_gain = None;
            let mut foot_planted_gain = None;
            let mut foot_raised_gain = None;
            let mut foot_unlock_gain = None;
            let mut world_from_model_feedback_gain = None;
            let mut error_up_down_bias = None;
            let mut align_world_from_model_gain = None;
            let mut hip_orientation_gain = None;
            let mut max_knee_angle_difference = None;
            let mut ankle_orientation_gain = None;


        for _value in _values {
            match _value {
                HkbFootIkGainsVisitor::OnOffGain(m) => on_off_gain = Some(m),
                HkbFootIkGainsVisitor::GroundAscendingGain(m) => ground_ascending_gain = Some(m),
                HkbFootIkGainsVisitor::GroundDescendingGain(m) => ground_descending_gain = Some(m),
                HkbFootIkGainsVisitor::FootPlantedGain(m) => foot_planted_gain = Some(m),
                HkbFootIkGainsVisitor::FootRaisedGain(m) => foot_raised_gain = Some(m),
                HkbFootIkGainsVisitor::FootUnlockGain(m) => foot_unlock_gain = Some(m),
                HkbFootIkGainsVisitor::WorldFromModelFeedbackGain(m) => world_from_model_feedback_gain = Some(m),
                HkbFootIkGainsVisitor::ErrorUpDownBias(m) => error_up_down_bias = Some(m),
                HkbFootIkGainsVisitor::AlignWorldFromModelGain(m) => align_world_from_model_gain = Some(m),
                HkbFootIkGainsVisitor::HipOrientationGain(m) => hip_orientation_gain = Some(m),
                HkbFootIkGainsVisitor::MaxKneeAngleDifference(m) => max_knee_angle_difference = Some(m),
                HkbFootIkGainsVisitor::AnkleOrientationGain(m) => ankle_orientation_gain = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            on_off_gain: on_off_gain.unwrap_or_default().into_inner(),
            ground_ascending_gain: ground_ascending_gain.unwrap_or_default().into_inner(),
            ground_descending_gain: ground_descending_gain.unwrap_or_default().into_inner(),
            foot_planted_gain: foot_planted_gain.unwrap_or_default().into_inner(),
            foot_raised_gain: foot_raised_gain.unwrap_or_default().into_inner(),
            foot_unlock_gain: foot_unlock_gain.unwrap_or_default().into_inner(),
            world_from_model_feedback_gain: world_from_model_feedback_gain.unwrap_or_default().into_inner(),
            error_up_down_bias: error_up_down_bias.unwrap_or_default().into_inner(),
            align_world_from_model_gain: align_world_from_model_gain.unwrap_or_default().into_inner(),
            hip_orientation_gain: hip_orientation_gain.unwrap_or_default().into_inner(),
            max_knee_angle_difference: max_knee_angle_difference.unwrap_or_default().into_inner(),
            ankle_orientation_gain: ankle_orientation_gain.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbFootIkGains> for Vec<HkbFootIkGainsVisitor> {
    fn from(data: &HkbFootIkGains) -> Self {
        vec![
            HkbFootIkGainsVisitor::OnOffGain(data.on_off_gain.into()),
            HkbFootIkGainsVisitor::GroundAscendingGain(data.ground_ascending_gain.into()),
            HkbFootIkGainsVisitor::GroundDescendingGain(data.ground_descending_gain.into()),
            HkbFootIkGainsVisitor::FootPlantedGain(data.foot_planted_gain.into()),
            HkbFootIkGainsVisitor::FootRaisedGain(data.foot_raised_gain.into()),
            HkbFootIkGainsVisitor::FootUnlockGain(data.foot_unlock_gain.into()),
            HkbFootIkGainsVisitor::WorldFromModelFeedbackGain(data.world_from_model_feedback_gain.into()),
            HkbFootIkGainsVisitor::ErrorUpDownBias(data.error_up_down_bias.into()),
            HkbFootIkGainsVisitor::AlignWorldFromModelGain(data.align_world_from_model_gain.into()),
            HkbFootIkGainsVisitor::HipOrientationGain(data.hip_orientation_gain.into()),
            HkbFootIkGainsVisitor::MaxKneeAngleDifference(data.max_knee_angle_difference.into()),
            HkbFootIkGainsVisitor::AnkleOrientationGain(data.ankle_orientation_gain.into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbFootIkGains {
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
enum HkbFootIkGainsVisitor {
    /// Visitor fields
    #[serde(rename = "onOffGain")]
    OnOffGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "groundAscendingGain")]
    GroundAscendingGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "groundDescendingGain")]
    GroundDescendingGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "footPlantedGain")]
    FootPlantedGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "footRaisedGain")]
    FootRaisedGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "footUnlockGain")]
    FootUnlockGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "worldFromModelFeedbackGain")]
    WorldFromModelFeedbackGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "errorUpDownBias")]
    ErrorUpDownBias(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "alignWorldFromModelGain")]
    AlignWorldFromModelGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "hipOrientationGain")]
    HipOrientationGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxKneeAngleDifference")]
    MaxKneeAngleDifference(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "ankleOrientationGain")]
    AnkleOrientationGain(Primitive<f32>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkGainsVisitor, "@name",
    ("onOffGain" => OnOffGain(Primitive<f32>)),
    ("groundAscendingGain" => GroundAscendingGain(Primitive<f32>)),
    ("groundDescendingGain" => GroundDescendingGain(Primitive<f32>)),
    ("footPlantedGain" => FootPlantedGain(Primitive<f32>)),
    ("footRaisedGain" => FootRaisedGain(Primitive<f32>)),
    ("footUnlockGain" => FootUnlockGain(Primitive<f32>)),
    ("worldFromModelFeedbackGain" => WorldFromModelFeedbackGain(Primitive<f32>)),
    ("errorUpDownBias" => ErrorUpDownBias(Primitive<f32>)),
    ("alignWorldFromModelGain" => AlignWorldFromModelGain(Primitive<f32>)),
    ("hipOrientationGain" => HipOrientationGain(Primitive<f32>)),
    ("maxKneeAngleDifference" => MaxKneeAngleDifference(Primitive<f32>)),
    ("ankleOrientationGain" => AnkleOrientationGain(Primitive<f32>)),
}
