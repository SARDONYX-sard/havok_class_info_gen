//! A Rust structure that implements a serializer/deserializer corresponding to `hkpShapeInfo`, a class defined in C++
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
/// -    size: 112
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpShapeInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpShapeInfo"`: Name of this class.
    #[serde(default = "HkpShapeInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xea7f1d08`: Unique value of this class.
    #[serde(default = "HkpShapeInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpShapeInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpShapeInfoHkParam<'a>>
}

impl HkpShapeInfo<'_> {
    /// Return `"hkpShapeInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpShapeInfo".into()
    }

    /// Return `"0xea7f1d08"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xea7f1d08".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpShapeInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shape")]
    Shape(Box<HkpShape>),
    /// # Information on fields in the original C++ class
    /// -   name:`"isHierarchicalCompound"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isHierarchicalCompound")]
    IsHierarchicalCompound(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"hkdShapesCollected"`
    /// -   type: `hkBool`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hkdShapesCollected")]
    HkdShapesCollected(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"childShapeNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childShapeNames")]
    ChildShapeNames(Vec<String>),
    /// # Information on fields in the original C++ class
    /// -   name:`"childTransforms"`
    /// -   type: `hkArray&lt;hkTransform&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childTransforms")]
    ChildTransforms(Vec<cgmath::Matrix4<f32>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transform")]
    Transform(cgmath::Matrix4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpShapeInfoHkParam<'de>, "@name",
    ("shape" => Shape(Box<HkpShape>)),
    ("isHierarchicalCompound" => IsHierarchicalCompound(bool)),
    ("hkdShapesCollected" => HkdShapesCollected(bool)),
    ("childShapeNames" => ChildShapeNames(Vec<String>)),
    ("childTransforms" => ChildTransforms(Vec<cgmath::Matrix4<f32>>)),
    ("transform" => Transform(cgmath::Matrix4<f32>)),
}
