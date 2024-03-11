//! A Rust structure that implements a serializer/deserializer corresponding to `hkpListShape`, a class defined in C++
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
/// -    size: 112
/// -  vtable: true
/// -  parent: hkpShapeCollection/`e8c3991d`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpListShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpListShape"`: The original C++ class name.
    #[serde(default = "HkpListShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa1937cbd`: Unique value of this class.
    #[serde(default = "HkpListShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpListShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpListShapeHkParam<'a>>
}

impl HkpListShape<'_> {
    /// Return `"hkpListShape"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpListShape".into()
    }

    /// Return `"0xa1937cbd"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa1937cbd".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpListShapeHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"childInfo"`
    /// -   type: `hkArray&lt;struct hkpListShapeChildInfo&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "childInfo")]
    ChildInfo(Vec<HkpListShapeChildInfo>),
    /// # Field information in the original C++ class
    /// -   name:`"flags"`
    /// -   type: `hkUint16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flags")]
    Flags(Primitive<u16>),
    /// # Field information in the original C++ class
    /// -   name:`"numDisabledChildren"`
    /// -   type: `hkUint16`
    /// - offset: 38
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numDisabledChildren")]
    NumDisabledChildren(Primitive<u16>),
    /// # Field information in the original C++ class
    /// -   name:`"aabbHalfExtents"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabbHalfExtents")]
    AabbHalfExtents(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"aabbCenter"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabbCenter")]
    AabbCenter(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"enabledChildren"`
    /// -   type: `hkUint32[8]`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enabledChildren")]
    EnabledChildren([Primitive<u32>; 8]),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpListShapeHkParam<'de>, "@name",
    ("childInfo" => ChildInfo(Vec<HkpListShapeChildInfo>)),
    ("flags" => Flags(Primitive<u16>)),
    ("numDisabledChildren" => NumDisabledChildren(Primitive<u16>)),
    ("aabbHalfExtents" => AabbHalfExtents(Vector4<f32>)),
    ("aabbCenter" => AabbCenter(Vector4<f32>)),
    ("enabledChildren" => EnabledChildren([Primitive<u32>; 8])),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ListShapeFlags {
    #[serde(rename = "ALL_FLAGS_CLEAR")]
    AllFlagsClear = 0,
    #[serde(rename = "DISABLE_SPU_CACHE_FOR_LIST_CHILD_INFO")]
    DisableSpuCacheForListChildInfo = 1,
}
