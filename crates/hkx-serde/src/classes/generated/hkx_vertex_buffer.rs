//! A Rust structure that implements a serializer/deserializer corresponding to `hkxVertexBuffer`, a class defined in C++
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
/// -    size: 104
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxVertexBuffer<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxVertexBuffer"`: Name of this class.
    #[serde(default = "HkxVertexBuffer::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x4ab10615`: Unique value of this class.
    #[serde(default = "HkxVertexBuffer::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxVertexBufferHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxVertexBufferHkParam<'a>>
}

impl HkxVertexBuffer<'_> {
    /// Return `"hkxVertexBuffer"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkxVertexBuffer".into()
    }

    /// Return `"0x4ab10615"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x4ab10615".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxVertexBufferHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"data"`
    /// -   type: `struct hkxVertexBufferVertexData`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(HkxVertexBufferVertexData),
    /// # Information on fields in the original C++ class
    /// -   name:`"desc"`
    /// -   type: `struct hkxVertexDescription`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "desc")]
    Desc(HkxVertexDescription),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxVertexBufferHkParam<'de>, "@name",
    ("data" => Data(HkxVertexBufferVertexData)),
    ("desc" => Desc(HkxVertexDescription)),
}
