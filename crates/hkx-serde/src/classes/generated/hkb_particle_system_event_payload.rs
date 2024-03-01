//! A Rust structure that implements a serializer/deserializer corresponding to `hkbParticleSystemEventPayload`, a class defined in C++
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
/// -  parent: hkbEventPayload/`da8c7d7d`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbParticleSystemEventPayload<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbParticleSystemEventPayload"`: Name of this class.
    #[serde(default = "HkbParticleSystemEventPayload::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x9df46cd6`: Unique value of this class.
    #[serde(default = "HkbParticleSystemEventPayload::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbParticleSystemEventPayloadHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbParticleSystemEventPayloadHkParam<'a>>
}

impl HkbParticleSystemEventPayload<'_> {
    /// Return `"hkbParticleSystemEventPayload"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbParticleSystemEventPayload".into()
    }

    /// Return `"0x9df46cd6"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x9df46cd6".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbParticleSystemEventPayloadHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum SystemType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(SystemType),
    /// # Information on fields in the original C++ class
    /// -   name:`"emitBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "emitBoneIndex")]
    EmitBoneIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"offset"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offset")]
    Offset(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"direction"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "direction")]
    Direction(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"numParticles"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numParticles")]
    NumParticles(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"speed"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "speed")]
    Speed(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbParticleSystemEventPayloadHkParam<'de>, "@name",
    ("type" => Type(SystemType)),
    ("emitBoneIndex" => EmitBoneIndex(i16)),
    ("offset" => Offset(cgmath::Vector4<f32>)),
    ("direction" => Direction(cgmath::Vector4<f32>)),
    ("numParticles" => NumParticles(i32)),
    ("speed" => Speed(f64)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SystemType {
    #[serde(rename = "DEBRIS")]
    Debris = 0,
    #[serde(rename = "DUST")]
    Dust = 1,
    #[serde(rename = "EXPLOSION")]
    Explosion = 2,
    #[serde(rename = "SMOKE")]
    Smoke = 3,
    #[serde(rename = "SPARKS")]
    Sparks = 4,
}
