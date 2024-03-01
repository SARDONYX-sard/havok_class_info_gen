//! A Rust structure that implements a serializer/deserializer corresponding to `hkbGetUpModifier`, a class defined in C++
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
/// -    size: 96
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbGetUpModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbGetUpModifier"`: Name of this class.
    #[serde(default = "HkbGetUpModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x61cb7ac0`: Unique value of this class.
    #[serde(default = "HkbGetUpModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbGetUpModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbGetUpModifierHkParam<'a>>
}

impl HkbGetUpModifier<'_> {
    /// Return `"hkbGetUpModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbGetUpModifier".into()
    }

    /// Return `"0x61cb7ac0"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x61cb7ac0".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbGetUpModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"groundNormal"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "groundNormal")]
    GroundNormal(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "duration")]
    Duration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"alignWithGroundDuration"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alignWithGroundDuration")]
    AlignWithGroundDuration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"rootBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rootBoneIndex")]
    RootBoneIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"otherBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 74
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "otherBoneIndex")]
    OtherBoneIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"anotherBoneIndex"`
    /// -   type: `hkInt16`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "anotherBoneIndex")]
    AnotherBoneIndex(i16),
    /// # Information on fields in the original C++ class
    /// -   name:`"timeSinceBegin"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeSinceBegin", skip_serializing)]
    TimeSinceBegin(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timeStep", skip_serializing)]
    TimeStep(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"initNextModify"`
    /// -   type: `hkBool`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "initNextModify", skip_serializing)]
    InitNextModify(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbGetUpModifierHkParam<'de>, "@name",
    ("groundNormal" => GroundNormal(cgmath::Vector4<f32>)),
    ("duration" => Duration(f64)),
    ("alignWithGroundDuration" => AlignWithGroundDuration(f64)),
    ("rootBoneIndex" => RootBoneIndex(i16)),
    ("otherBoneIndex" => OtherBoneIndex(i16)),
    ("anotherBoneIndex" => AnotherBoneIndex(i16)),
    ("timeSinceBegin" => TimeSinceBegin(f64)),
    ("timeStep" => TimeStep(f64)),
    ("initNextModify" => InitNextModify(bool)),
}
