//! A Rust structure that implements a serializer/deserializer corresponding to `hkpMultiSphereShape`, a class defined in C++
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
/// -    size: 160
/// -  vtable: true
/// -  parent: hkpSphereRepShape/`e7eca7eb`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpMultiSphereShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpMultiSphereShape"`: Name of this class.
    #[serde(default = "HkpMultiSphereShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x61a590fc`: Unique value of this class.
    #[serde(default = "HkpMultiSphereShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpMultiSphereShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpMultiSphereShapeHkParam<'a>>
}

impl HkpMultiSphereShape<'_> {
    /// Return `"hkpMultiSphereShape"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpMultiSphereShape".into()
    }

    /// Return `"0x61a590fc"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x61a590fc".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMultiSphereShapeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"numSpheres"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numSpheres")]
    NumSpheres(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"spheres"`
    /// -   type: `hkVector4[8]`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "spheres")]
    Spheres([cgmath::Vector4<f32>; 8]),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpMultiSphereShapeHkParam<'de>, "@name",
    ("numSpheres" => NumSpheres(i32)),
    ("spheres" => Spheres([cgmath::Vector4<f32>; 8])),
}
