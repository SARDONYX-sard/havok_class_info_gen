//! A Rust structure that implements a serializer/deserializer corresponding to `hkaDefaultAnimatedReferenceFrame`, a class defined in C++
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
/// -    size: 64
/// -  vtable: true
/// -  parent: hkaAnimatedReferenceFrame/`da8c7d7d`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaDefaultAnimatedReferenceFrame<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaDefaultAnimatedReferenceFrame"`: Name of this class.
    #[serde(default = "HkaDefaultAnimatedReferenceFrame::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x6d85e445`: Unique value of this class.
    #[serde(default = "HkaDefaultAnimatedReferenceFrame::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaDefaultAnimatedReferenceFrameHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaDefaultAnimatedReferenceFrameHkParam<'a>>
}

impl HkaDefaultAnimatedReferenceFrame<'_> {
    /// Return `"hkaDefaultAnimatedReferenceFrame"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkaDefaultAnimatedReferenceFrame".into()
    }

    /// Return `"0x6d85e445"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x6d85e445".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaDefaultAnimatedReferenceFrameHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"up"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "up")]
    Up(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"forward"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forward")]
    Forward(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "duration")]
    Duration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"referenceFrameSamples"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "referenceFrameSamples")]
    ReferenceFrameSamples(Vec<cgmath::Vector4<f32>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaDefaultAnimatedReferenceFrameHkParam<'de>, "@name",
    ("up" => Up(cgmath::Vector4<f32>)),
    ("forward" => Forward(cgmath::Vector4<f32>)),
    ("duration" => Duration(f64)),
    ("referenceFrameSamples" => ReferenceFrameSamples(Vec<cgmath::Vector4<f32>>)),
}
