//! A Rust structure that implements a serializer/deserializer corresponding to `hkpRagdollConstraintDataAtoms`, a class defined in C++
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
/// -    size: 336
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpRagdollConstraintDataAtoms<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpRagdollConstraintDataAtoms"`: The original C++ class name.
    #[serde(default = "HkpRagdollConstraintDataAtoms::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xeed76b00`: Unique value of this class.
    #[serde(default = "HkpRagdollConstraintDataAtoms::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpRagdollConstraintDataAtomsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpRagdollConstraintDataAtomsHkParam<'a>>
}

impl HkpRagdollConstraintDataAtoms<'_> {
    /// Return `"hkpRagdollConstraintDataAtoms"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpRagdollConstraintDataAtoms".into()
    }

    /// Return `"0xeed76b00"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xeed76b00".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRagdollConstraintDataAtomsHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"transforms"`
    /// -   type: `struct hkpSetLocalTransformsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transforms")]
    Transforms(HkpSetLocalTransformsConstraintAtom),
    /// # Field information in the original C++ class
    /// -   name:`"setupStabilization"`
    /// -   type: `struct hkpSetupStabilizationAtom`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setupStabilization")]
    SetupStabilization(HkpSetupStabilizationAtom),
    /// # Field information in the original C++ class
    /// -   name:`"ragdollMotors"`
    /// -   type: `struct hkpRagdollMotorConstraintAtom`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ragdollMotors")]
    RagdollMotors(HkpRagdollMotorConstraintAtom),
    /// # Field information in the original C++ class
    /// -   name:`"angFriction"`
    /// -   type: `struct hkpAngFrictionConstraintAtom`
    /// - offset: 240
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angFriction")]
    AngFriction(HkpAngFrictionConstraintAtom),
    /// # Field information in the original C++ class
    /// -   name:`"twistLimit"`
    /// -   type: `struct hkpTwistLimitConstraintAtom`
    /// - offset: 252
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "twistLimit")]
    TwistLimit(HkpTwistLimitConstraintAtom),
    /// # Field information in the original C++ class
    /// -   name:`"coneLimit"`
    /// -   type: `struct hkpConeLimitConstraintAtom`
    /// - offset: 272
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "coneLimit")]
    ConeLimit(HkpConeLimitConstraintAtom),
    /// # Field information in the original C++ class
    /// -   name:`"planesLimit"`
    /// -   type: `struct hkpConeLimitConstraintAtom`
    /// - offset: 292
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "planesLimit")]
    PlanesLimit(HkpConeLimitConstraintAtom),
    /// # Field information in the original C++ class
    /// -   name:`"ballSocket"`
    /// -   type: `struct hkpBallSocketConstraintAtom`
    /// - offset: 312
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ballSocket")]
    BallSocket(HkpBallSocketConstraintAtom),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpRagdollConstraintDataAtomsHkParam<'de>, "@name",
    ("transforms" => Transforms(HkpSetLocalTransformsConstraintAtom)),
    ("setupStabilization" => SetupStabilization(HkpSetupStabilizationAtom)),
    ("ragdollMotors" => RagdollMotors(HkpRagdollMotorConstraintAtom)),
    ("angFriction" => AngFriction(HkpAngFrictionConstraintAtom)),
    ("twistLimit" => TwistLimit(HkpTwistLimitConstraintAtom)),
    ("coneLimit" => ConeLimit(HkpConeLimitConstraintAtom)),
    ("planesLimit" => PlanesLimit(HkpConeLimitConstraintAtom)),
    ("ballSocket" => BallSocket(HkpBallSocketConstraintAtom)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Axis {
    #[serde(rename = "AXIS_TWIST")]
    AxisTwist = 0,
    #[serde(rename = "AXIS_PLANES")]
    AxisPlanes = 1,
    #[serde(rename = "AXIS_CROSS_PRODUCT")]
    AxisCrossProduct = 2,
}
