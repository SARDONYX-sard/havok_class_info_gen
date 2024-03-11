//! A Rust structure that implements a serializer/deserializer corresponding to `hkpCompressedSampledHeightFieldShape`, a class defined in C++
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
/// -    size: 128
/// -  vtable: true
/// -  parent: hkpSampledHeightFieldShape/`11213421`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpCompressedSampledHeightFieldShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpCompressedSampledHeightFieldShape"`: The original C++ class name.
    #[serde(default = "HkpCompressedSampledHeightFieldShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x97b6e143`: Unique value of this class.
    #[serde(default = "HkpCompressedSampledHeightFieldShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpCompressedSampledHeightFieldShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpCompressedSampledHeightFieldShapeHkParam<'a>>
}

impl HkpCompressedSampledHeightFieldShape<'_> {
    /// Return `"hkpCompressedSampledHeightFieldShape"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpCompressedSampledHeightFieldShape".into()
    }

    /// Return `"0x97b6e143"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x97b6e143".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCompressedSampledHeightFieldShapeHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"storage"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "storage")]
    Storage(Vec<Primitive<u16>>),
    /// # Field information in the original C++ class
    /// -   name:`"triangleFlip"`
    /// -   type: `hkBool`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triangleFlip")]
    TriangleFlip(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"offset"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offset")]
    Offset(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"scale"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scale")]
    Scale(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpCompressedSampledHeightFieldShapeHkParam<'de>, "@name",
    ("storage" => Storage(Vec<Primitive<u16>>)),
    ("triangleFlip" => TriangleFlip(Primitive<bool>)),
    ("offset" => Offset(Primitive<f32>)),
    ("scale" => Scale(Primitive<f32>)),
}
