//! A Rust structure that implements a serializer/deserializer corresponding to `hkpRagdollLimitsDataAtoms`, a class defined in C++
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
/// -    size: 176
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpRagdollLimitsDataAtoms<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpRagdollLimitsDataAtoms"`: The original C++ class name.
    #[serde(default = "HkpRagdollLimitsDataAtoms::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x82b894c3`: Unique value of this class.
    #[serde(default = "HkpRagdollLimitsDataAtoms::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpRagdollLimitsDataAtomsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpRagdollLimitsDataAtomsHkParam<'a>>
}

impl HkpRagdollLimitsDataAtoms<'_> {
    /// Return `"hkpRagdollLimitsDataAtoms"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpRagdollLimitsDataAtoms".into()
    }

    /// Return `"0x82b894c3"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x82b894c3".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRagdollLimitsDataAtomsHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"rotations"`
    /// -   type: `struct hkpSetLocalRotationsConstraintAtom`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotations")]
    Rotations(HkpSetLocalRotationsConstraintAtom),
    /// # Field information in the original C++ class
    /// -   name:`"twistLimit"`
    /// -   type: `struct hkpTwistLimitConstraintAtom`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "twistLimit")]
    TwistLimit(HkpTwistLimitConstraintAtom),
    /// # Field information in the original C++ class
    /// -   name:`"coneLimit"`
    /// -   type: `struct hkpConeLimitConstraintAtom`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "coneLimit")]
    ConeLimit(HkpConeLimitConstraintAtom),
    /// # Field information in the original C++ class
    /// -   name:`"planesLimit"`
    /// -   type: `struct hkpConeLimitConstraintAtom`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "planesLimit")]
    PlanesLimit(HkpConeLimitConstraintAtom),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpRagdollLimitsDataAtomsHkParam<'de>, "@name",
    ("rotations" => Rotations(HkpSetLocalRotationsConstraintAtom)),
    ("twistLimit" => TwistLimit(HkpTwistLimitConstraintAtom)),
    ("coneLimit" => ConeLimit(HkpConeLimitConstraintAtom)),
    ("planesLimit" => PlanesLimit(HkpConeLimitConstraintAtom)),
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
