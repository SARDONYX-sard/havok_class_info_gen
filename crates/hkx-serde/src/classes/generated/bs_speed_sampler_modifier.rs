//! A Rust structure that implements a serializer/deserializer corresponding to `BSSpeedSamplerModifier`, a class defined in C++
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
/// -    size: 60
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsSpeedSamplerModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSSpeedSamplerModifier"`: Name of this class.
    #[serde(default = "BsSpeedSamplerModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xd297fda9`: Unique value of this class.
    #[serde(default = "BsSpeedSamplerModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsSpeedSamplerModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsSpeedSamplerModifierHkParam<'a>>
}

impl BsSpeedSamplerModifier<'_> {
    /// Return `"BSSpeedSamplerModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BsSpeedSamplerModifier".into()
    }

    /// Return `"0xd297fda9"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xd297fda9".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsSpeedSamplerModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"state"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "state")]
    State(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"direction"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "direction")]
    Direction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"goalSpeed"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "goalSpeed")]
    GoalSpeed(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"speedOut"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "speedOut")]
    SpeedOut(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsSpeedSamplerModifierHkParam<'de>, "@name",
    ("state" => State(i32)),
    ("direction" => Direction(f64)),
    ("goalSpeed" => GoalSpeed(f64)),
    ("speedOut" => SpeedOut(f64)),
}
