//! A Rust structure that implements a serializer/deserializer corresponding to `hkxMeshSection`, a class defined in C++
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
/// -    size: 40
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxMeshSection<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxMeshSection"`: The original C++ class name.
    #[serde(default = "HkxMeshSection::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xe2286cf8`: Unique value of this class.
    #[serde(default = "HkxMeshSection::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxMeshSectionHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxMeshSectionHkParam<'a>>
}

impl HkxMeshSection<'_> {
    /// Return `"hkxMeshSection"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkxMeshSection".into()
    }

    /// Return `"0xe2286cf8"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xe2286cf8".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxMeshSectionHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"vertexBuffer"`
    /// -   type: `struct hkxVertexBuffer*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBuffer")]
    VertexBuffer(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"indexBuffers"`
    /// -   type: `hkArray&lt;hkxIndexBuffer*&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexBuffers")]
    IndexBuffers(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"material"`
    /// -   type: `struct hkxMaterial*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "material")]
    Material(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"userChannels"`
    /// -   type: `hkArray&lt;hkReferencedObject*&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userChannels")]
    UserChannels(Vec<Cow<'a, str>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxMeshSectionHkParam<'de>, "@name",
    ("vertexBuffer" => VertexBuffer(Cow<'a, str>)),
    ("indexBuffers" => IndexBuffers(Vec<Cow<'a, str>>)),
    ("material" => Material(Cow<'a, str>)),
    ("userChannels" => UserChannels(Vec<Cow<'a, str>>)),
}
