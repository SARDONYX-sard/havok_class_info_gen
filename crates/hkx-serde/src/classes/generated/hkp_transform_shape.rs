//! A Rust structure that implements a serializer/deserializer corresponding to `hkpTransformShape`, a class defined in C++
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
/// -  parent: hkpShape/`666490a1`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpTransformShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpTransformShape"`: Name of this class.
    #[serde(default = "HkpTransformShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x787ef513`: Unique value of this class.
    #[serde(default = "HkpTransformShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpTransformShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpTransformShapeHkParam<'a>>
}

impl HkpTransformShape<'_> {
    /// Return `"hkpTransformShape"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpTransformShape".into()
    }

    /// Return `"0x787ef513"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x787ef513".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpTransformShapeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"childShape"`
    /// -   type: `struct hkpSingleShapeContainer`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childShape")]
    ChildShape(HkpSingleShapeContainer),
    /// # Information on fields in the original C++ class
    /// -   name:`"childShapeSize"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "childShapeSize", skip_serializing)]
    ChildShapeSize(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"rotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotation")]
    Rotation(cgmath::Quaternion<f32>),
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
    HkpTransformShapeHkParam<'de>, "@name",
    ("childShape" => ChildShape(HkpSingleShapeContainer)),
    ("childShapeSize" => ChildShapeSize(i32)),
    ("rotation" => Rotation(cgmath::Quaternion<f32>)),
    ("transform" => Transform(cgmath::Matrix4<f32>)),
}
