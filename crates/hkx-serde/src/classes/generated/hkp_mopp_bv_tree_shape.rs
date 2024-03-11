//! A Rust structure that implements a serializer/deserializer corresponding to `hkpMoppBvTreeShape`, a class defined in C++
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
/// -    size: 64
/// -  vtable: true
/// -  parent: hkMoppBvTreeShapeBase/`7c338c66`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpMoppBvTreeShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpMoppBvTreeShape"`: The original C++ class name.
    #[serde(default = "HkpMoppBvTreeShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x90b29d39`: Unique value of this class.
    #[serde(default = "HkpMoppBvTreeShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpMoppBvTreeShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpMoppBvTreeShapeHkParam<'a>>
}

impl HkpMoppBvTreeShape<'_> {
    /// Return `"hkpMoppBvTreeShape"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpMoppBvTreeShape".into()
    }

    /// Return `"0x90b29d39"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x90b29d39".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMoppBvTreeShapeHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"child"`
    /// -   type: `struct hkpSingleShapeContainer`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "child")]
    Child(HkpSingleShapeContainer),
    /// # Field information in the original C++ class
    /// -   name:`"childSize"`
    /// -   type: `hkInt32`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "childSize", skip_serializing)]
    ChildSize(Primitive<i32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpMoppBvTreeShapeHkParam<'de>, "@name",
    ("child" => Child(HkpSingleShapeContainer)),
    ("childSize" => ChildSize(Primitive<i32>)),
}
