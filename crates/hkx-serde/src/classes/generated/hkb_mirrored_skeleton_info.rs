//! A Rust structure that implements a serializer/deserializer corresponding to `hkbMirroredSkeletonInfo`, a class defined in C++
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
/// -    size: 48
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbMirroredSkeletonInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbMirroredSkeletonInfo"`: Name of this class.
    #[serde(default = "HkbMirroredSkeletonInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xc6c2da4f`: Unique value of this class.
    #[serde(default = "HkbMirroredSkeletonInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbMirroredSkeletonInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbMirroredSkeletonInfoHkParam<'a>>
}

impl HkbMirroredSkeletonInfo<'_> {
    /// Return `"hkbMirroredSkeletonInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbMirroredSkeletonInfo".into()
    }

    /// Return `"0xc6c2da4f"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc6c2da4f".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbMirroredSkeletonInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"mirrorAxis"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mirrorAxis")]
    MirrorAxis(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"bonePairMap"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bonePairMap")]
    BonePairMap(Vec<i16>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbMirroredSkeletonInfoHkParam<'de>, "@name",
    ("mirrorAxis" => MirrorAxis(cgmath::Vector4<f32>)),
    ("bonePairMap" => BonePairMap(Vec<i16>)),
}
