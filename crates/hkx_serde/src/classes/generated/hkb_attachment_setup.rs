//! Rust [`serde::Serializer`]/[`serde::Deserializer`] corresponding to C++ class `hkbAttachmentSetup`
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

/// `hkbAttachmentSetup`
///
/// - In C++, it represents the name of one field in the class.
/// - In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// # C++ Class Info
/// -      size: 40
/// -    vtable: true
/// -    parent: `hkReferencedObject`/`0x3b1c1113`
/// - signature: `0x774632b`
/// -   version: 1
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HkbAttachmentSetup {
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"memSizeAndFlags"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub mem_size_and_flags: u16,
    /// # C++ Parent class(`hkReferencedObject` => parent: `hkBaseObject`) field Info
    /// -   name:`"referenceCount"`
    /// -   type: `hkInt16`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE|SERIALIZE_IGNORED`
    pub reference_count: i16,

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// # C++ Class Fields Info
    /// -   name:`"blendInTime"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    pub blend_in_time: f32,
    /// # C++ Class Fields Info
    /// -   name:`"moveAttacherFraction"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    pub move_attacher_fraction: f32,
    /// # C++ Class Fields Info
    /// -   name:`"gain"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    pub gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"extrapolationTimeStep"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    pub extrapolation_time_step: f32,
    /// # C++ Class Fields Info
    /// -   name:`"fixUpGain"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    pub fix_up_gain: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxLinearDistance"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    pub max_linear_distance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"maxAngularDistance"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    pub max_angular_distance: f32,
    /// # C++ Class Fields Info
    /// -   name:`"attachmentType"`
    /// -   type: `enum AttachmentType`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    pub attachment_type: AttachmentType,
}

impl Serialize for HkbAttachmentSetup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let visitor: Vec<HkbAttachmentSetupVisitor> = self.into();
        visitor.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HkbAttachmentSetup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use `Vec` instead, because the fields of this class are more than 32 and serde only supports up to `[T; 32]`.
        let de = <Vec<HkbAttachmentSetupVisitor>>::deserialize(deserializer)?;
        Ok(de.into())
    }
}

impl From<Vec<HkbAttachmentSetupVisitor>> for HkbAttachmentSetup {
    fn from(_values: Vec<HkbAttachmentSetupVisitor>) -> Self {
            let mut mem_size_and_flags = None;
            let mut reference_count = None;
            let mut blend_in_time = None;
            let mut move_attacher_fraction = None;
            let mut gain = None;
            let mut extrapolation_time_step = None;
            let mut fix_up_gain = None;
            let mut max_linear_distance = None;
            let mut max_angular_distance = None;
            let mut attachment_type = None;


        for _value in _values {
            match _value {
                HkbAttachmentSetupVisitor::MemSizeAndFlags(m) => mem_size_and_flags = Some(m),
                HkbAttachmentSetupVisitor::ReferenceCount(m) => reference_count = Some(m),
                HkbAttachmentSetupVisitor::BlendInTime(m) => blend_in_time = Some(m),
                HkbAttachmentSetupVisitor::MoveAttacherFraction(m) => move_attacher_fraction = Some(m),
                HkbAttachmentSetupVisitor::Gain(m) => gain = Some(m),
                HkbAttachmentSetupVisitor::ExtrapolationTimeStep(m) => extrapolation_time_step = Some(m),
                HkbAttachmentSetupVisitor::FixUpGain(m) => fix_up_gain = Some(m),
                HkbAttachmentSetupVisitor::MaxLinearDistance(m) => max_linear_distance = Some(m),
                HkbAttachmentSetupVisitor::MaxAngularDistance(m) => max_angular_distance = Some(m),
                HkbAttachmentSetupVisitor::AttachmentType(m) => attachment_type = Some(m),

            }
        }

        // This `unwrap_or_default` is never called because it depends on the default value of `Visitor
        Self {
            mem_size_and_flags: mem_size_and_flags.unwrap_or_default().into_inner(),
            reference_count: reference_count.unwrap_or_default().into_inner(),
            blend_in_time: blend_in_time.unwrap_or_default().into_inner(),
            move_attacher_fraction: move_attacher_fraction.unwrap_or_default().into_inner(),
            gain: gain.unwrap_or_default().into_inner(),
            extrapolation_time_step: extrapolation_time_step.unwrap_or_default().into_inner(),
            fix_up_gain: fix_up_gain.unwrap_or_default().into_inner(),
            max_linear_distance: max_linear_distance.unwrap_or_default().into_inner(),
            max_angular_distance: max_angular_distance.unwrap_or_default().into_inner(),
            attachment_type: attachment_type.unwrap_or_default().into_inner(),

        }
    }
}

// The only way to create a possessive type from a reference is to `clone` it.
// This `From` is only used for serialization, so this overhead is only incurred during serialization.
impl From<&HkbAttachmentSetup> for Vec<HkbAttachmentSetupVisitor> {
    fn from(data: &HkbAttachmentSetup) -> Self {
        vec![
            HkbAttachmentSetupVisitor::MemSizeAndFlags(data.mem_size_and_flags.into()),
            HkbAttachmentSetupVisitor::ReferenceCount(data.reference_count.into()),
            HkbAttachmentSetupVisitor::BlendInTime(data.blend_in_time.into()),
            HkbAttachmentSetupVisitor::MoveAttacherFraction(data.move_attacher_fraction.into()),
            HkbAttachmentSetupVisitor::Gain(data.gain.into()),
            HkbAttachmentSetupVisitor::ExtrapolationTimeStep(data.extrapolation_time_step.into()),
            HkbAttachmentSetupVisitor::FixUpGain(data.fix_up_gain.into()),
            HkbAttachmentSetupVisitor::MaxLinearDistance(data.max_linear_distance.into()),
            HkbAttachmentSetupVisitor::MaxAngularDistance(data.max_angular_distance.into()),
            HkbAttachmentSetupVisitor::AttachmentType(data.attachment_type.clone().into()),

        ]
    }
}

impl <'bytes: 'de, 'de> ByteDeSerialize<'bytes, 'de> for HkbAttachmentSetup {
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
enum HkbAttachmentSetupVisitor {
    /// Visitor fields
    #[serde(rename = "memSizeAndFlags", skip_serializing)]
    MemSizeAndFlags(Primitive<u16>),
    /// Visitor fields
    #[serde(rename = "referenceCount", skip_serializing)]
    ReferenceCount(Primitive<i16>),

    // C++ Parent class(`hkBaseObject` => parent: `None`) has no fields
    //
    /// Visitor fields
    #[serde(rename = "blendInTime")]
    BlendInTime(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "moveAttacherFraction")]
    MoveAttacherFraction(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "gain")]
    Gain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "extrapolationTimeStep")]
    ExtrapolationTimeStep(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "fixUpGain")]
    FixUpGain(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxLinearDistance")]
    MaxLinearDistance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "maxAngularDistance")]
    MaxAngularDistance(Primitive<f32>),
    /// Visitor fields
    #[serde(rename = "attachmentType")]
    AttachmentType(Primitive<AttachmentType>),
}

// Manual implementation to branch the process using the value of the `name` attribute as the key.
impl_deserialize_for_internally_tagged_enum! {
    HkbAttachmentSetupVisitor, "@name",
    ("memSizeAndFlags" => MemSizeAndFlags(Primitive<u16>)),
    ("referenceCount" => ReferenceCount(Primitive<i16>)),
    ("blendInTime" => BlendInTime(Primitive<f32>)),
    ("moveAttacherFraction" => MoveAttacherFraction(Primitive<f32>)),
    ("gain" => Gain(Primitive<f32>)),
    ("extrapolationTimeStep" => ExtrapolationTimeStep(Primitive<f32>)),
    ("fixUpGain" => FixUpGain(Primitive<f32>)),
    ("maxLinearDistance" => MaxLinearDistance(Primitive<f32>)),
    ("maxAngularDistance" => MaxAngularDistance(Primitive<f32>)),
    ("attachmentType" => AttachmentType(Primitive<AttachmentType>)),
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToPrimitive, FromPrimitive)]
pub enum AttachmentType {
    #[serde(rename = "ATTACHMENT_TYPE_KEYFRAME_RIGID_BODY")]
    #[default]
    AttachmentTypeKeyframeRigidBody = 0,
    #[serde(rename = "ATTACHMENT_TYPE_BALL_SOCKET_CONSTRAINT")]
    AttachmentTypeBallSocketConstraint = 1,
    #[serde(rename = "ATTACHMENT_TYPE_RAGDOLL_CONSTRAINT")]
    AttachmentTypeRagdollConstraint = 2,
    #[serde(rename = "ATTACHMENT_TYPE_SET_WORLD_FROM_MODEL")]
    AttachmentTypeSetWorldFromModel = 3,
    #[serde(rename = "ATTACHMENT_TYPE_NONE")]
    AttachmentTypeNone = 4,
}
