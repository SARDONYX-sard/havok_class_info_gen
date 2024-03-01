//! A Rust structure that implements a serializer/deserializer corresponding to `hkpPoweredChainDataConstraintInfo`, a class defined in C++
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
/// -    size: 80
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpPoweredChainDataConstraintInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpPoweredChainDataConstraintInfo"`: Name of this class.
    #[serde(default = "HkpPoweredChainDataConstraintInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf88aee25`: Unique value of this class.
    #[serde(default = "HkpPoweredChainDataConstraintInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpPoweredChainDataConstraintInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpPoweredChainDataConstraintInfoHkParam<'a>>
}

impl HkpPoweredChainDataConstraintInfo<'_> {
    /// Return `"hkpPoweredChainDataConstraintInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpPoweredChainDataConstraintInfo".into()
    }

    /// Return `"0xf88aee25"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf88aee25".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPoweredChainDataConstraintInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"pivotInA"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pivotInA")]
    PivotInA(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"pivotInB"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pivotInB")]
    PivotInB(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"aTc"`
    /// -   type: `hkQuaternion`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aTc")]
    ATc(cgmath::Quaternion<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"bTc"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bTc")]
    BTc(cgmath::Quaternion<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"motors"`
    /// -   type: `struct hkpConstraintMotor*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motors")]
    Motors(Box<HkpConstraintMotor>),
    /// # Information on fields in the original C++ class
    /// -   name:`"switchBodies"`
    /// -   type: `hkBool`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "switchBodies")]
    SwitchBodies(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpPoweredChainDataConstraintInfoHkParam<'de>, "@name",
    ("pivotInA" => PivotInA(cgmath::Vector4<f32>)),
    ("pivotInB" => PivotInB(cgmath::Vector4<f32>)),
    ("aTc" => ATc(cgmath::Quaternion<f32>)),
    ("bTc" => BTc(cgmath::Quaternion<f32>)),
    ("motors" => Motors(Box<HkpConstraintMotor>)),
    ("switchBodies" => SwitchBodies(bool)),
}
