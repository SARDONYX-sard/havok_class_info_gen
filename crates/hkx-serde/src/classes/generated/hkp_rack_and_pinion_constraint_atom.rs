//! A Rust structure that implements a serializer/deserializer corresponding to `hkpRackAndPinionConstraintAtom`, a class defined in C++
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
/// -    size: 12
/// -  vtable: false
/// -  parent: hkpConstraintAtom/`59d67ef6`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpRackAndPinionConstraintAtom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpRackAndPinionConstraintAtom"`: The original C++ class name.
    #[serde(default = "HkpRackAndPinionConstraintAtom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x30cae006`: Unique value of this class.
    #[serde(default = "HkpRackAndPinionConstraintAtom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpRackAndPinionConstraintAtomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpRackAndPinionConstraintAtomHkParam<'a>>
}

impl HkpRackAndPinionConstraintAtom<'_> {
    /// Return `"hkpRackAndPinionConstraintAtom"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpRackAndPinionConstraintAtom".into()
    }

    /// Return `"0x30cae006"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x30cae006".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpRackAndPinionConstraintAtomHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"pinionRadiusOrScrewPitch"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pinionRadiusOrScrewPitch")]
    PinionRadiusOrScrewPitch(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"isScrew"`
    /// -   type: `hkBool`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isScrew")]
    IsScrew(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"memOffsetToInitialAngleOffset"`
    /// -   type: `hkInt8`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memOffsetToInitialAngleOffset")]
    MemOffsetToInitialAngleOffset(Primitive<i8>),
    /// # Field information in the original C++ class
    /// -   name:`"memOffsetToPrevAngle"`
    /// -   type: `hkInt8`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memOffsetToPrevAngle")]
    MemOffsetToPrevAngle(Primitive<i8>),
    /// # Field information in the original C++ class
    /// -   name:`"memOffsetToRevolutionCounter"`
    /// -   type: `hkInt8`
    /// - offset: 11
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memOffsetToRevolutionCounter")]
    MemOffsetToRevolutionCounter(Primitive<i8>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpRackAndPinionConstraintAtomHkParam<'de>, "@name",
    ("pinionRadiusOrScrewPitch" => PinionRadiusOrScrewPitch(Primitive<f32>)),
    ("isScrew" => IsScrew(Primitive<bool>)),
    ("memOffsetToInitialAngleOffset" => MemOffsetToInitialAngleOffset(Primitive<i8>)),
    ("memOffsetToPrevAngle" => MemOffsetToPrevAngle(Primitive<i8>)),
    ("memOffsetToRevolutionCounter" => MemOffsetToRevolutionCounter(Primitive<i8>)),
}
