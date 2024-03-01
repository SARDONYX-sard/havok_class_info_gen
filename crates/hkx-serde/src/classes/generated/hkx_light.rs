//! A Rust structure that implements a serializer/deserializer corresponding to `hkxLight`, a class defined in C++
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
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxLight<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxLight"`: Name of this class.
    #[serde(default = "HkxLight::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x81c86d42`: Unique value of this class.
    #[serde(default = "HkxLight::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxLightHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxLightHkParam<'a>>
}

impl HkxLight<'_> {
    /// Return `"hkxLight"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkxLight".into()
    }

    /// Return `"0x81c86d42"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x81c86d42".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxLightHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum LightType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(LightType),
    /// # Information on fields in the original C++ class
    /// -   name:`"position"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "position")]
    Position(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"direction"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "direction")]
    Direction(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"color"`
    /// -   type: `hkUint32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "color")]
    Color(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"angle"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angle")]
    Angle(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxLightHkParam<'de>, "@name",
    ("type" => Type(LightType)),
    ("position" => Position(cgmath::Vector4<f32>)),
    ("direction" => Direction(cgmath::Vector4<f32>)),
    ("color" => Color(u32)),
    ("angle" => Angle(f64)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LightType {
    #[serde(rename = "POINT_LIGHT")]
    PointLight = 0,
    #[serde(rename = "DIRECTIONAL_LIGHT")]
    DirectionalLight = 1,
    #[serde(rename = "SPOT_LIGHT")]
    SpotLight = 2,
}
