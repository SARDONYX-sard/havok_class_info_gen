//! A Rust structure that implements a serializer/deserializer corresponding to `hkpMultiRayShape`, a class defined in C++
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
/// -    size: 32
/// -  vtable: true
/// -  parent: hkpShape/`666490a1`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpMultiRayShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpMultiRayShape"`: Name of this class.
    #[serde(default = "HkpMultiRayShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xea2e7ec9`: Unique value of this class.
    #[serde(default = "HkpMultiRayShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpMultiRayShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpMultiRayShapeHkParam<'a>>
}

impl HkpMultiRayShape<'_> {
    /// Return `"hkpMultiRayShape"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpMultiRayShape".into()
    }

    /// Return `"0xea2e7ec9"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xea2e7ec9".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMultiRayShapeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"rays"`
    /// -   type: `hkArray&lt;struct hkpMultiRayShapeRay&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rays")]
    Rays(Vec<HkpMultiRayShapeRay>),
    /// # Information on fields in the original C++ class
    /// -   name:`"rayPenetrationDistance"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rayPenetrationDistance")]
    RayPenetrationDistance(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpMultiRayShapeHkParam<'de>, "@name",
    ("rays" => Rays(Vec<HkpMultiRayShapeRay>)),
    ("rayPenetrationDistance" => RayPenetrationDistance(f64)),
}
