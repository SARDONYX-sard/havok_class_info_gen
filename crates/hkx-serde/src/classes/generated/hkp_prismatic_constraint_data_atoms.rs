//! A Rust structure that implements a serializer/deserializer corresponding to `hkpPrismaticConstraintDataAtoms`, a class defined in C++
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
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpPrismaticConstraintDataAtoms<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpPrismaticConstraintDataAtoms"`: Name of this class.
    #[serde(default = "HkpPrismaticConstraintDataAtoms::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x7f516137`: Unique value of this class.
    #[serde(default = "HkpPrismaticConstraintDataAtoms::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpPrismaticConstraintDataAtomsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpPrismaticConstraintDataAtomsHkParam<'a>>
}

impl HkpPrismaticConstraintDataAtoms<'_> {
    /// Return `"hkpPrismaticConstraintDataAtoms"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpPrismaticConstraintDataAtoms".into()
    }

    /// Return `"0x7f516137"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x7f516137".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPrismaticConstraintDataAtomsHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"transforms"`
    /// -   type: `struct hkpSetLocalTransformsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transforms")]
    Transforms(HkpSetLocalTransformsConstraintAtom),
    /// # Information on fields in the original C++ class
    /// -   name:`"motor"`
    /// -   type: `struct hkpLinMotorConstraintAtom`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motor")]
    Motor(HkpLinMotorConstraintAtom),
    /// # Information on fields in the original C++ class
    /// -   name:`"friction"`
    /// -   type: `struct hkpLinFrictionConstraintAtom`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "friction")]
    Friction(HkpLinFrictionConstraintAtom),
    /// # Information on fields in the original C++ class
    /// -   name:`"ang"`
    /// -   type: `struct hkpAngConstraintAtom`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ang")]
    Ang(HkpAngConstraintAtom),
    /// # Information on fields in the original C++ class
    /// -   name:`"lin0"`
    /// -   type: `struct hkpLinConstraintAtom`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lin0")]
    Lin0(HkpLinConstraintAtom),
    /// # Information on fields in the original C++ class
    /// -   name:`"lin1"`
    /// -   type: `struct hkpLinConstraintAtom`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lin1")]
    Lin1(HkpLinConstraintAtom),
    /// # Information on fields in the original C++ class
    /// -   name:`"linLimit"`
    /// -   type: `struct hkpLinLimitConstraintAtom`
    /// - offset: 180
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "linLimit")]
    LinLimit(HkpLinLimitConstraintAtom),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpPrismaticConstraintDataAtomsHkParam<'de>, "@name",
    ("transforms" => Transforms(HkpSetLocalTransformsConstraintAtom)),
    ("motor" => Motor(HkpLinMotorConstraintAtom)),
    ("friction" => Friction(HkpLinFrictionConstraintAtom)),
    ("ang" => Ang(HkpAngConstraintAtom)),
    ("lin0" => Lin0(HkpLinConstraintAtom)),
    ("lin1" => Lin1(HkpLinConstraintAtom)),
    ("linLimit" => LinLimit(HkpLinLimitConstraintAtom)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Axis {
    #[serde(rename = "AXIS_SHAFT")]
    AxisShaft = 0,
    #[serde(rename = "AXIS_PERP_TO_SHAFT")]
    AxisPerpToShaft = 1,
}
