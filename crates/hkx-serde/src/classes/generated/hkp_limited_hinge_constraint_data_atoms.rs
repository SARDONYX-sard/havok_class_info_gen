//! A Rust structure that implements a serializer/deserializer corresponding to `hkpLimitedHingeConstraintDataAtoms`, a class defined in C++
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
/// -    size: 240
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpLimitedHingeConstraintDataAtoms<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpLimitedHingeConstraintDataAtoms"`: The original C++ class name.
    #[serde(default = "HkpLimitedHingeConstraintDataAtoms::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x54c7715b`: Unique value of this class.
    #[serde(default = "HkpLimitedHingeConstraintDataAtoms::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpLimitedHingeConstraintDataAtomsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpLimitedHingeConstraintDataAtomsHkParam<'a>>
}

impl HkpLimitedHingeConstraintDataAtoms<'_> {
    /// Return `"hkpLimitedHingeConstraintDataAtoms"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpLimitedHingeConstraintDataAtoms".into()
    }

    /// Return `"0x54c7715b"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x54c7715b".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpLimitedHingeConstraintDataAtomsHkParam<'a> {
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
    /// -   name:`"angMotor"`
    /// -   type: `struct hkpAngMotorConstraintAtom`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angMotor")]
    AngMotor(HkpAngMotorConstraintAtom),
    /// # Field information in the original C++ class
    /// -   name:`"angFriction"`
    /// -   type: `struct hkpAngFrictionConstraintAtom`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angFriction")]
    AngFriction(HkpAngFrictionConstraintAtom),
    /// # Field information in the original C++ class
    /// -   name:`"angLimit"`
    /// -   type: `struct hkpAngLimitConstraintAtom`
    /// - offset: 192
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angLimit")]
    AngLimit(HkpAngLimitConstraintAtom),
    /// # Field information in the original C++ class
    /// -   name:`"2dAng"`
    /// -   type: `struct hkp2dAngConstraintAtom`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "2dAng")]
    2DAng(Hkp2DAngConstraintAtom),
    /// # Field information in the original C++ class
    /// -   name:`"ballSocket"`
    /// -   type: `struct hkpBallSocketConstraintAtom`
    /// - offset: 212
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ballSocket")]
    BallSocket(HkpBallSocketConstraintAtom),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpLimitedHingeConstraintDataAtomsHkParam<'de>, "@name",
    ("transforms" => Transforms(HkpSetLocalTransformsConstraintAtom)),
    ("setupStabilization" => SetupStabilization(HkpSetupStabilizationAtom)),
    ("angMotor" => AngMotor(HkpAngMotorConstraintAtom)),
    ("angFriction" => AngFriction(HkpAngFrictionConstraintAtom)),
    ("angLimit" => AngLimit(HkpAngLimitConstraintAtom)),
    ("2dAng" => 2DAng(Hkp2DAngConstraintAtom)),
    ("ballSocket" => BallSocket(HkpBallSocketConstraintAtom)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Axis {
    #[serde(rename = "AXIS_AXLE")]
    AxisAxle = 0,
    #[serde(rename = "AXIS_PERP_TO_AXLE_1")]
    AxisPerpToAxle1 = 1,
    #[serde(rename = "AXIS_PERP_TO_AXLE_2")]
    AxisPerpToAxle2 = 2,
}
