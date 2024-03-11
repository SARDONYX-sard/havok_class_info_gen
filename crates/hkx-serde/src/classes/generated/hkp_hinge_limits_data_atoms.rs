//! A Rust structure that implements a serializer/deserializer corresponding to `hkpHingeLimitsDataAtoms`, a class defined in C++
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
/// -    size: 144
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpHingeLimitsDataAtoms<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpHingeLimitsDataAtoms"`: The original C++ class name.
    #[serde(default = "HkpHingeLimitsDataAtoms::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x555876ff`: Unique value of this class.
    #[serde(default = "HkpHingeLimitsDataAtoms::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpHingeLimitsDataAtomsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpHingeLimitsDataAtomsHkParam<'a>>
}

impl HkpHingeLimitsDataAtoms<'_> {
    /// Return `"hkpHingeLimitsDataAtoms"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpHingeLimitsDataAtoms".into()
    }

    /// Return `"0x555876ff"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x555876ff".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpHingeLimitsDataAtomsHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"rotations"`
    /// -   type: `struct hkpSetLocalRotationsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotations")]
    Rotations(HkpSetLocalRotationsConstraintAtom),
    /// # Field information in the original C++ class
    /// -   name:`"angLimit"`
    /// -   type: `struct hkpAngLimitConstraintAtom`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angLimit")]
    AngLimit(HkpAngLimitConstraintAtom),
    /// # Field information in the original C++ class
    /// -   name:`"2dAng"`
    /// -   type: `struct hkp2dAngConstraintAtom`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "2dAng")]
    2DAng(Hkp2DAngConstraintAtom),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpHingeLimitsDataAtomsHkParam<'de>, "@name",
    ("rotations" => Rotations(HkpSetLocalRotationsConstraintAtom)),
    ("angLimit" => AngLimit(HkpAngLimitConstraintAtom)),
    ("2dAng" => 2DAng(Hkp2DAngConstraintAtom)),
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
