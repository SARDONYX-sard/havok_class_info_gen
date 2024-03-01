//! A Rust structure that implements a serializer/deserializer corresponding to `hkpConvexListShape`, a class defined in C++
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
/// -  vtable: true
/// -  parent: hkpConvexShape/`f8f74f85`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpConvexListShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpConvexListShape"`: Name of this class.
    #[serde(default = "HkpConvexListShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x450b26e8`: Unique value of this class.
    #[serde(default = "HkpConvexListShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpConvexListShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpConvexListShapeHkParam<'a>>
}

impl HkpConvexListShape<'_> {
    /// Return `"hkpConvexListShape"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpConvexListShape".into()
    }

    /// Return `"0x450b26e8"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x450b26e8".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConvexListShapeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"minDistanceToUseConvexHullForGetClosestPoints"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "minDistanceToUseConvexHullForGetClosestPoints")]
    MinDistanceToUseConvexHullForGetClosestPoints(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"aabbHalfExtents"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabbHalfExtents")]
    AabbHalfExtents(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"aabbCenter"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabbCenter")]
    AabbCenter(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"useCachedAabb"`
    /// -   type: `hkBool`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useCachedAabb")]
    UseCachedAabb(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"childShapes"`
    /// -   type: `hkArray&lt;hkpConvexShape*&gt;`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childShapes")]
    ChildShapes(Vec<Box<HkpConvexShape>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexListShapeHkParam<'de>, "@name",
    ("minDistanceToUseConvexHullForGetClosestPoints" => MinDistanceToUseConvexHullForGetClosestPoints(f64)),
    ("aabbHalfExtents" => AabbHalfExtents(cgmath::Vector4<f32>)),
    ("aabbCenter" => AabbCenter(cgmath::Vector4<f32>)),
    ("useCachedAabb" => UseCachedAabb(bool)),
    ("childShapes" => ChildShapes(Vec<Box<HkpConvexShape>>)),
}
