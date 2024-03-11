//! A Rust structure that implements a serializer/deserializer corresponding to `hkxAttribute`, a class defined in C++
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
/// -    size: 8
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxAttribute<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxAttribute"`: The original C++ class name.
    #[serde(default = "HkxAttribute::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x7375cae3`: Unique value of this class.
    #[serde(default = "HkxAttribute::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxAttributeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxAttributeHkParam<'a>>
}

impl HkxAttribute<'_> {
    /// Return `"hkxAttribute"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkxAttribute".into()
    }

    /// Return `"0x7375cae3"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x7375cae3".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxAttributeHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"value"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "value")]
    Value(Cow<'a, str>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxAttributeHkParam<'de>, "@name",
    ("name" => Name(Primitive<Cow<'a, str>>)),
    ("value" => Value(Cow<'a, str>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Hint {
    #[serde(rename = "HINT_NONE")]
    HintNone = 0,
    #[serde(rename = "HINT_IGNORE")]
    HintIgnore = 1,
    #[serde(rename = "HINT_TRANSFORM")]
    HintTransform = 2,
    #[serde(rename = "HINT_SCALE")]
    HintScale = 4,
    #[serde(rename = "HINT_TRANSFORM_AND_SCALE")]
    HintTransformAndScale = 6,
    #[serde(rename = "HINT_FLIP")]
    HintFlip = 8,
}
