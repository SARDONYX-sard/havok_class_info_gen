//! A Rust structure that implements a serializer/deserializer corresponding to `hkbRegisteredGenerator`, a class defined in C++
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
/// -  parent: hkbBindable/`2c1432d7`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbRegisteredGenerator<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbRegisteredGenerator"`: Name of this class.
    #[serde(default = "HkbRegisteredGenerator::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x58b1d082`: Unique value of this class.
    #[serde(default = "HkbRegisteredGenerator::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbRegisteredGeneratorHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbRegisteredGeneratorHkParam<'a>>
}

impl HkbRegisteredGenerator<'_> {
    /// Return `"hkbRegisteredGenerator"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbRegisteredGenerator".into()
    }

    /// Return `"0x58b1d082"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x58b1d082".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbRegisteredGeneratorHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"generator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "generator")]
    Generator(Box<HkbGenerator>),
    /// # Information on fields in the original C++ class
    /// -   name:`"relativePosition"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "relativePosition")]
    RelativePosition(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"relativeDirection"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "relativeDirection")]
    RelativeDirection(cgmath::Vector4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbRegisteredGeneratorHkParam<'de>, "@name",
    ("generator" => Generator(Box<HkbGenerator>)),
    ("relativePosition" => RelativePosition(cgmath::Vector4<f32>)),
    ("relativeDirection" => RelativeDirection(cgmath::Vector4<f32>)),
}
