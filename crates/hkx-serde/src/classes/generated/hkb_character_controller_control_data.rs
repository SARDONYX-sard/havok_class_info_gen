//! A Rust structure that implements a serializer/deserializer corresponding to `hkbCharacterControllerControlData`, a class defined in C++
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
/// -    size: 32
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbCharacterControllerControlData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbCharacterControllerControlData"`: The original C++ class name.
    #[serde(default = "HkbCharacterControllerControlData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x5b6c03d9`: Unique value of this class.
    #[serde(default = "HkbCharacterControllerControlData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbCharacterControllerControlDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbCharacterControllerControlDataHkParam<'a>>
}

impl HkbCharacterControllerControlData<'_> {
    /// Return `"hkbCharacterControllerControlData"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbCharacterControllerControlData".into()
    }

    /// Return `"0x5b6c03d9"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x5b6c03d9".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterControllerControlDataHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"desiredVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "desiredVelocity")]
    DesiredVelocity(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"verticalGain"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalGain")]
    VerticalGain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"horizontalCatchUpGain"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "horizontalCatchUpGain")]
    HorizontalCatchUpGain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxVerticalSeparation"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxVerticalSeparation")]
    MaxVerticalSeparation(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxHorizontalSeparation"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxHorizontalSeparation")]
    MaxHorizontalSeparation(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterControllerControlDataHkParam<'de>, "@name",
    ("desiredVelocity" => DesiredVelocity(Vector4<f32>)),
    ("verticalGain" => VerticalGain(Primitive<f32>)),
    ("horizontalCatchUpGain" => HorizontalCatchUpGain(Primitive<f32>)),
    ("maxVerticalSeparation" => MaxVerticalSeparation(Primitive<f32>)),
    ("maxHorizontalSeparation" => MaxHorizontalSeparation(Primitive<f32>)),
}
