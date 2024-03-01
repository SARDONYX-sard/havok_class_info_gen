//! A Rust structure that implements a serializer/deserializer corresponding to `BSDistTriggerModifier`, a class defined in C++
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
/// -    size: 80
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsDistTriggerModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSDistTriggerModifier"`: Name of this class.
    #[serde(default = "BsDistTriggerModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xb34d2bbd`: Unique value of this class.
    #[serde(default = "BsDistTriggerModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsDistTriggerModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsDistTriggerModifierHkParam<'a>>
}

impl BsDistTriggerModifier<'_> {
    /// Return `"BSDistTriggerModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BsDistTriggerModifier".into()
    }

    /// Return `"0xb34d2bbd"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xb34d2bbd".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsDistTriggerModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"targetPosition"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetPosition")]
    TargetPosition(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"distance"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "distance")]
    Distance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"distanceTrigger"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "distanceTrigger")]
    DistanceTrigger(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"triggerEvent"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triggerEvent")]
    TriggerEvent(HkbEventProperty),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsDistTriggerModifierHkParam<'de>, "@name",
    ("targetPosition" => TargetPosition(cgmath::Vector4<f32>)),
    ("distance" => Distance(f64)),
    ("distanceTrigger" => DistanceTrigger(f64)),
    ("triggerEvent" => TriggerEvent(HkbEventProperty)),
}
