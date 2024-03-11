//! A Rust structure that implements a serializer/deserializer corresponding to `hkbAttachmentSetup`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use crate::hk_types::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 40
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbAttachmentSetup<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbAttachmentSetup"`: The original C++ class name.
    #[serde(default = "HkbAttachmentSetup::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x774632b`: Unique value of this class.
    #[serde(default = "HkbAttachmentSetup::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbAttachmentSetupHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbAttachmentSetupHkParam<'a>>
}

impl HkbAttachmentSetup<'_> {
    /// Return `"hkbAttachmentSetup"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbAttachmentSetup".into()
    }

    /// Return `"0x774632b"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x774632b".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbAttachmentSetupHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"blendInTime"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blendInTime")]
    BlendInTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"moveAttacherFraction"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "moveAttacherFraction")]
    MoveAttacherFraction(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"gain"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "gain")]
    Gain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"extrapolationTimeStep"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extrapolationTimeStep")]
    ExtrapolationTimeStep(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"fixUpGain"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fixUpGain")]
    FixUpGain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxLinearDistance"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxLinearDistance")]
    MaxLinearDistance(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxAngularDistance"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxAngularDistance")]
    MaxAngularDistance(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"attachmentType"`
    /// -   type: `enum AttachmentType`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attachmentType")]
    AttachmentType(AttachmentType),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbAttachmentSetupHkParam<'de>, "@name",
    ("blendInTime" => BlendInTime(Primitive<f32>)),
    ("moveAttacherFraction" => MoveAttacherFraction(Primitive<f32>)),
    ("gain" => Gain(Primitive<f32>)),
    ("extrapolationTimeStep" => ExtrapolationTimeStep(Primitive<f32>)),
    ("fixUpGain" => FixUpGain(Primitive<f32>)),
    ("maxLinearDistance" => MaxLinearDistance(Primitive<f32>)),
    ("maxAngularDistance" => MaxAngularDistance(Primitive<f32>)),
    ("attachmentType" => AttachmentType(AttachmentType)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AttachmentType {
    #[serde(rename = "ATTACHMENT_TYPE_KEYFRAME_RIGID_BODY")]
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
