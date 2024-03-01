//! A Rust structure that implements a serializer/deserializer corresponding to `hkpWheelConstraintDataAtoms`, a class defined in C++
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
/// -    size: 304
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpWheelConstraintDataAtoms<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpWheelConstraintDataAtoms"`: Name of this class.
    #[serde(default = "HkpWheelConstraintDataAtoms::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x1188cbe1`: Unique value of this class.
    #[serde(default = "HkpWheelConstraintDataAtoms::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpWheelConstraintDataAtomsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpWheelConstraintDataAtomsHkParam<'a>>
}

impl HkpWheelConstraintDataAtoms<'_> {
    /// Return `"hkpWheelConstraintDataAtoms"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpWheelConstraintDataAtoms".into()
    }

    /// Return `"0x1188cbe1"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x1188cbe1".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpWheelConstraintDataAtomsHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"suspensionBase"`
    /// -   type: `struct hkpSetLocalTransformsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "suspensionBase")]
    SuspensionBase(HkpSetLocalTransformsConstraintAtom),
    /// # Information on fields in the original C++ class
    /// -   name:`"lin0Limit"`
    /// -   type: `struct hkpLinLimitConstraintAtom`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lin0Limit")]
    Lin0Limit(HkpLinLimitConstraintAtom),
    /// # Information on fields in the original C++ class
    /// -   name:`"lin0Soft"`
    /// -   type: `struct hkpLinSoftConstraintAtom`
    /// - offset: 156
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lin0Soft")]
    Lin0Soft(HkpLinSoftConstraintAtom),
    /// # Information on fields in the original C++ class
    /// -   name:`"lin1"`
    /// -   type: `struct hkpLinConstraintAtom`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lin1")]
    Lin1(HkpLinConstraintAtom),
    /// # Information on fields in the original C++ class
    /// -   name:`"lin2"`
    /// -   type: `struct hkpLinConstraintAtom`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lin2")]
    Lin2(HkpLinConstraintAtom),
    /// # Information on fields in the original C++ class
    /// -   name:`"steeringBase"`
    /// -   type: `struct hkpSetLocalRotationsConstraintAtom`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "steeringBase")]
    SteeringBase(HkpSetLocalRotationsConstraintAtom),
    /// # Information on fields in the original C++ class
    /// -   name:`"2dAng"`
    /// -   type: `struct hkp2dAngConstraintAtom`
    /// - offset: 288
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "2dAng")]
    2DAng(Hkp2DAngConstraintAtom),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpWheelConstraintDataAtomsHkParam<'de>, "@name",
    ("suspensionBase" => SuspensionBase(HkpSetLocalTransformsConstraintAtom)),
    ("lin0Limit" => Lin0Limit(HkpLinLimitConstraintAtom)),
    ("lin0Soft" => Lin0Soft(HkpLinSoftConstraintAtom)),
    ("lin1" => Lin1(HkpLinConstraintAtom)),
    ("lin2" => Lin2(HkpLinConstraintAtom)),
    ("steeringBase" => SteeringBase(HkpSetLocalRotationsConstraintAtom)),
    ("2dAng" => 2DAng(Hkp2DAngConstraintAtom)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Axis {
    #[serde(rename = "AXIS_SUSPENSION")]
    AxisSuspension = 0,
    #[serde(rename = "AXIS_PERP_SUSPENSION")]
    AxisPerpSuspension = 1,
    #[serde(rename = "AXIS_AXLE")]
    AxisAxle = 0,
    #[serde(rename = "AXIS_STEERING")]
    AxisSteering = 1,
}
