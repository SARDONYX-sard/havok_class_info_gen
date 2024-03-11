//! A Rust structure that implements a serializer/deserializer corresponding to `hkbFootIkControlsModifierLeg`, a class defined in C++
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
pub struct HkbFootIkControlsModifierLeg<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbFootIkControlsModifierLeg"`: The original C++ class name.
    #[serde(default = "HkbFootIkControlsModifierLeg::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x9e17091a`: Unique value of this class.
    #[serde(default = "HkbFootIkControlsModifierLeg::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbFootIkControlsModifierLegHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbFootIkControlsModifierLegHkParam<'a>>
}

impl HkbFootIkControlsModifierLeg<'_> {
    /// Return `"hkbFootIkControlsModifierLeg"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbFootIkControlsModifierLeg".into()
    }

    /// Return `"0x9e17091a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x9e17091a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbFootIkControlsModifierLegHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"groundPosition"`
    /// -   type: `hkVector4`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "groundPosition")]
    GroundPosition(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"ungroundedEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ungroundedEvent")]
    UngroundedEvent(HkbEventProperty),
    /// # Field information in the original C++ class
    /// -   name:`"verticalError"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalError")]
    VerticalError(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"hitSomething"`
    /// -   type: `hkBool`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hitSomething")]
    HitSomething(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"isPlantedMS"`
    /// -   type: `hkBool`
    /// - offset: 29
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isPlantedMS")]
    IsPlantedMs(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkControlsModifierLegHkParam<'de>, "@name",
    ("groundPosition" => GroundPosition(Vector4<f32>)),
    ("ungroundedEvent" => UngroundedEvent(HkbEventProperty)),
    ("verticalError" => VerticalError(Primitive<f32>)),
    ("hitSomething" => HitSomething(Primitive<bool>)),
    ("isPlantedMS" => IsPlantedMs(Primitive<bool>)),
}
