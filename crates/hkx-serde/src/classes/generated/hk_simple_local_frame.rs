//! A Rust structure that implements a serializer/deserializer corresponding to `hkSimpleLocalFrame`, a class defined in C++
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
/// -    size: 112
/// -  vtable: true
/// -  parent: hkLocalFrame/`da8c7d7d`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkSimpleLocalFrame<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkSimpleLocalFrame"`: Name of this class.
    #[serde(default = "HkSimpleLocalFrame::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xe758f63c`: Unique value of this class.
    #[serde(default = "HkSimpleLocalFrame::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkSimpleLocalFrameHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkSimpleLocalFrameHkParam<'a>>
}

impl HkSimpleLocalFrame<'_> {
    /// Return `"hkSimpleLocalFrame"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkSimpleLocalFrame".into()
    }

    /// Return `"0xe758f63c"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xe758f63c".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkSimpleLocalFrameHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transform")]
    Transform(cgmath::Matrix4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"children"`
    /// -   type: `hkArray&lt;hkLocalFrame*&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "children")]
    Children(Vec<Box<HkLocalFrame>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"parentFrame"`
    /// -   type: `struct hkLocalFrame*`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE | NOT_OWNED`
    #[serde(rename = "parentFrame")]
    ParentFrame(Box<HkLocalFrame>),
    /// # Information on fields in the original C++ class
    /// -   name:`"group"`
    /// -   type: `struct hkLocalFrameGroup*`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "group")]
    Group(Box<HkLocalFrameGroup>),
    /// # Information on fields in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(String),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkSimpleLocalFrameHkParam<'de>, "@name",
    ("transform" => Transform(cgmath::Matrix4<f32>)),
    ("children" => Children(Vec<Box<HkLocalFrame>>)),
    ("parentFrame" => ParentFrame(Box<HkLocalFrame>)),
    ("group" => Group(Box<HkLocalFrameGroup>)),
    ("name" => Name(String)),
}