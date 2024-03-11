//! A Rust structure that implements a serializer/deserializer corresponding to `hkbBoneIndexArray`, a class defined in C++
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
/// -  parent: hkbBindable/`2c1432d7`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbBoneIndexArray<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbBoneIndexArray"`: The original C++ class name.
    #[serde(default = "HkbBoneIndexArray::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xaa8619`: Unique value of this class.
    #[serde(default = "HkbBoneIndexArray::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbBoneIndexArrayHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbBoneIndexArrayHkParam<'a>>
}

impl HkbBoneIndexArray<'_> {
    /// Return `"hkbBoneIndexArray"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbBoneIndexArray".into()
    }

    /// Return `"0xaa8619"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xaa8619".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBoneIndexArrayHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"boneIndices"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneIndices")]
    BoneIndices(Vec<Primitive<i16>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbBoneIndexArrayHkParam<'de>, "@name",
    ("boneIndices" => BoneIndices(Vec<Primitive<i16>>)),
}
