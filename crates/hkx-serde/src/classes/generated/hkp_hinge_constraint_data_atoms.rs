//! A Rust structure that implements a serializer/deserializer corresponding to `hkpHingeConstraintDataAtoms`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 192
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpHingeConstraintDataAtoms<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpHingeConstraintDataAtoms"`: Name of this class.
    #[serde(default = "HkpHingeConstraintDataAtoms::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x6958371c`: Unique value of this class.
    #[serde(default = "HkpHingeConstraintDataAtoms::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpHingeConstraintDataAtomsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpHingeConstraintDataAtomsHkParam<'a>>
}

impl HkpHingeConstraintDataAtoms<'_> {
    /// Return `"hkpHingeConstraintDataAtoms"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpHingeConstraintDataAtoms".into()
    }

    /// Return `"0x6958371c"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x6958371c".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpHingeConstraintDataAtomsHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"transforms"`
    /// -   type: `struct hkpSetLocalTransformsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transforms")]
    Transforms(HkpSetLocalTransformsConstraintAtom),
    /// # Information on fields in the original C++ class
    /// -   name:`"setupStabilization"`
    /// -   type: `struct hkpSetupStabilizationAtom`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setupStabilization")]
    SetupStabilization(HkpSetupStabilizationAtom),
    /// # Information on fields in the original C++ class
    /// -   name:`"2dAng"`
    /// -   type: `struct hkp2dAngConstraintAtom`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "2dAng")]
    2DAng(Hkp2DAngConstraintAtom),
    /// # Information on fields in the original C++ class
    /// -   name:`"ballSocket"`
    /// -   type: `struct hkpBallSocketConstraintAtom`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ballSocket")]
    BallSocket(HkpBallSocketConstraintAtom),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpHingeConstraintDataAtomsHkParam<'de>, "@name",
    ("transforms" => Transforms(HkpSetLocalTransformsConstraintAtom)),
    ("setupStabilization" => SetupStabilization(HkpSetupStabilizationAtom)),
    ("2dAng" => 2DAng(Hkp2DAngConstraintAtom)),
    ("ballSocket" => BallSocket(HkpBallSocketConstraintAtom)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Axis {
    #[serde(rename = "AXIS_AXLE")]
    AxisAxle = 0,
}
