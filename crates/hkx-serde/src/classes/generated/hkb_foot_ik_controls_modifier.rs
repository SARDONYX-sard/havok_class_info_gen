//! A Rust structure that implements a serializer/deserializer corresponding to `hkbFootIkControlsModifier`, a class defined in C++
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
/// -    size: 144
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbFootIkControlsModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbFootIkControlsModifier"`: The original C++ class name.
    #[serde(default = "HkbFootIkControlsModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xe5b6f544`: Unique value of this class.
    #[serde(default = "HkbFootIkControlsModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbFootIkControlsModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbFootIkControlsModifierHkParam<'a>>
}

impl HkbFootIkControlsModifier<'_> {
    /// Return `"hkbFootIkControlsModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbFootIkControlsModifier".into()
    }

    /// Return `"0xe5b6f544"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xe5b6f544".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbFootIkControlsModifierHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"controlData"`
    /// -   type: `struct hkbFootIkControlData`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "controlData")]
    ControlData(HkbFootIkControlData),
    /// # Field information in the original C++ class
    /// -   name:`"legs"`
    /// -   type: `hkArray&lt;struct hkbFootIkControlsModifierLeg&gt;`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "legs")]
    Legs(Vec<HkbFootIkControlsModifierLeg>),
    /// # Field information in the original C++ class
    /// -   name:`"errorOutTranslation"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "errorOutTranslation")]
    ErrorOutTranslation(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"alignWithGroundRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alignWithGroundRotation")]
    AlignWithGroundRotation(Quaternion<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkControlsModifierHkParam<'de>, "@name",
    ("controlData" => ControlData(HkbFootIkControlData)),
    ("legs" => Legs(Vec<HkbFootIkControlsModifierLeg>)),
    ("errorOutTranslation" => ErrorOutTranslation(Vector4<f32>)),
    ("alignWithGroundRotation" => AlignWithGroundRotation(Quaternion<f32>)),
}
