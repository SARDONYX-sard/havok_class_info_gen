//! A Rust structure that implements a serializer/deserializer corresponding to `hkpConvexTransformShapeBase`, a class defined in C++
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
/// -    size: 32
/// -  vtable: true
/// -  parent: hkpConvexShape/`f8f74f85`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpConvexTransformShapeBase<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpConvexTransformShapeBase"`: The original C++ class name.
    #[serde(default = "HkpConvexTransformShapeBase::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xfbd72f9`: Unique value of this class.
    #[serde(default = "HkpConvexTransformShapeBase::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpConvexTransformShapeBaseHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpConvexTransformShapeBaseHkParam<'a>>
}

impl HkpConvexTransformShapeBase<'_> {
    /// Return `"hkpConvexTransformShapeBase"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpConvexTransformShapeBase".into()
    }

    /// Return `"0xfbd72f9"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xfbd72f9".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConvexTransformShapeBaseHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"childShape"`
    /// -   type: `struct hkpSingleShapeContainer`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childShape")]
    ChildShape(HkpSingleShapeContainer),
    /// # Field information in the original C++ class
    /// -   name:`"childShapeSize"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "childShapeSize", skip_serializing)]
    ChildShapeSize(Primitive<i32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexTransformShapeBaseHkParam<'de>, "@name",
    ("childShape" => ChildShape(HkpSingleShapeContainer)),
    ("childShapeSize" => ChildShapeSize(Primitive<i32>)),
}
