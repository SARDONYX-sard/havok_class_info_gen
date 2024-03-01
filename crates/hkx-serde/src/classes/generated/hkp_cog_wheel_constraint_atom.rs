//! A Rust structure that implements a serializer/deserializer corresponding to `hkpCogWheelConstraintAtom`, a class defined in C++
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
/// -    size: 16
/// -  vtable: false
/// -  parent: hkpConstraintAtom/`59d67ef6`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpCogWheelConstraintAtom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpCogWheelConstraintAtom"`: Name of this class.
    #[serde(default = "HkpCogWheelConstraintAtom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf2b1f399`: Unique value of this class.
    #[serde(default = "HkpCogWheelConstraintAtom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpCogWheelConstraintAtomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpCogWheelConstraintAtomHkParam<'a>>
}

impl HkpCogWheelConstraintAtom<'_> {
    /// Return `"hkpCogWheelConstraintAtom"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpCogWheelConstraintAtom".into()
    }

    /// Return `"0xf2b1f399"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf2b1f399".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCogWheelConstraintAtomHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"cogWheelRadiusA"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cogWheelRadiusA")]
    CogWheelRadiusA(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"cogWheelRadiusB"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cogWheelRadiusB")]
    CogWheelRadiusB(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"isScrew"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isScrew")]
    IsScrew(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"memOffsetToInitialAngleOffset"`
    /// -   type: `hkInt8`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memOffsetToInitialAngleOffset")]
    MemOffsetToInitialAngleOffset(i8),
    /// # Information on fields in the original C++ class
    /// -   name:`"memOffsetToPrevAngle"`
    /// -   type: `hkInt8`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memOffsetToPrevAngle")]
    MemOffsetToPrevAngle(i8),
    /// # Information on fields in the original C++ class
    /// -   name:`"memOffsetToRevolutionCounter"`
    /// -   type: `hkInt8`
    /// - offset: 15
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "memOffsetToRevolutionCounter")]
    MemOffsetToRevolutionCounter(i8),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpCogWheelConstraintAtomHkParam<'de>, "@name",
    ("cogWheelRadiusA" => CogWheelRadiusA(f64)),
    ("cogWheelRadiusB" => CogWheelRadiusB(f64)),
    ("isScrew" => IsScrew(bool)),
    ("memOffsetToInitialAngleOffset" => MemOffsetToInitialAngleOffset(i8)),
    ("memOffsetToPrevAngle" => MemOffsetToPrevAngle(i8)),
    ("memOffsetToRevolutionCounter" => MemOffsetToRevolutionCounter(i8)),
}
