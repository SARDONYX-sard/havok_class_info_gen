//! A Rust structure that implements a serializer/deserializer corresponding to `hkpWheelConstraintData`, a class defined in C++
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
/// -    size: 352
/// -  vtable: true
/// -  parent: hkpConstraintData/`80559a4e`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpWheelConstraintData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpWheelConstraintData"`: Name of this class.
    #[serde(default = "HkpWheelConstraintData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xb4c46671`: Unique value of this class.
    #[serde(default = "HkpWheelConstraintData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpWheelConstraintDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpWheelConstraintDataHkParam<'a>>
}

impl HkpWheelConstraintData<'_> {
    /// Return `"hkpWheelConstraintData"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpWheelConstraintData".into()
    }

    /// Return `"0xb4c46671"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xb4c46671".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpWheelConstraintDataHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"atoms"`
    /// -   type: `struct hkpWheelConstraintDataAtoms`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "atoms")]
    Atoms(HkpWheelConstraintDataAtoms),
    /// # Information on fields in the original C++ class
    /// -   name:`"initialAxleInB"`
    /// -   type: `hkVector4`
    /// - offset: 320
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initialAxleInB")]
    InitialAxleInB(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"initialSteeringAxisInB"`
    /// -   type: `hkVector4`
    /// - offset: 336
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initialSteeringAxisInB")]
    InitialSteeringAxisInB(cgmath::Vector4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpWheelConstraintDataHkParam<'de>, "@name",
    ("atoms" => Atoms(HkpWheelConstraintDataAtoms)),
    ("initialAxleInB" => InitialAxleInB(cgmath::Vector4<f32>)),
    ("initialSteeringAxisInB" => InitialSteeringAxisInB(cgmath::Vector4<f32>)),
}
