//! A Rust structure that implements a serializer/deserializer corresponding to `hkpCdBody`, a class defined in C++
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
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpCdBody<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpCdBody"`: Name of this class.
    #[serde(default = "HkpCdBody::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x54a4b841`: Unique value of this class.
    #[serde(default = "HkpCdBody::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpCdBodyHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpCdBodyHkParam<'a>>
}

impl HkpCdBody<'_> {
    /// Return `"hkpCdBody"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpCdBody".into()
    }

    /// Return `"0x54a4b841"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x54a4b841".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCdBodyHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shape")]
    Shape(Box<HkpShape>),
    /// # Information on fields in the original C++ class
    /// -   name:`"shapeKey"`
    /// -   type: `hkUint32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shapeKey")]
    ShapeKey(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"motion"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "motion", skip_serializing)]
    Motion(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"parent"`
    /// -   type: `struct hkpCdBody*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "parent", skip_serializing)]
    Parent(Box<HkpCdBody>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpCdBodyHkParam<'de>, "@name",
    ("shape" => Shape(Box<HkpShape>)),
    ("shapeKey" => ShapeKey(u32)),
    ("motion" => Motion(())),
    ("parent" => Parent(Box<HkpCdBody>)),
}
