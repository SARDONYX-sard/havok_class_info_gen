//! A Rust structure that implements a serializer/deserializer corresponding to `hkpMaterial`, a class defined in C++
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
/// -    size: 12
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpMaterial<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpMaterial"`: The original C++ class name.
    #[serde(default = "HkpMaterial::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x33be6570`: Unique value of this class.
    #[serde(default = "HkpMaterial::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpMaterialHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpMaterialHkParam<'a>>
}

impl HkpMaterial<'_> {
    /// Return `"hkpMaterial"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpMaterial".into()
    }

    /// Return `"0x33be6570"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x33be6570".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMaterialHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"responseType"`
    /// -   type: `enum ResponseType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "responseType")]
    ResponseType(ResponseType),
    /// # Field information in the original C++ class
    /// -   name:`"rollingFrictionMultiplier"`
    /// -   type: `hkHalf`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rollingFrictionMultiplier")]
    RollingFrictionMultiplier(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"friction"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "friction")]
    Friction(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"restitution"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "restitution")]
    Restitution(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpMaterialHkParam<'de>, "@name",
    ("responseType" => ResponseType(ResponseType)),
    ("rollingFrictionMultiplier" => RollingFrictionMultiplier(Primitive<f32>)),
    ("friction" => Friction(Primitive<f32>)),
    ("restitution" => Restitution(Primitive<f32>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseType {
    #[serde(rename = "RESPONSE_INVALID")]
    ResponseInvalid = 0,
    #[serde(rename = "RESPONSE_SIMPLE_CONTACT")]
    ResponseSimpleContact = 1,
    #[serde(rename = "RESPONSE_REPORTING")]
    ResponseReporting = 2,
    #[serde(rename = "RESPONSE_NONE")]
    ResponseNone = 3,
    #[serde(rename = "RESPONSE_MAX_ID")]
    ResponseMaxId = 4,
}
