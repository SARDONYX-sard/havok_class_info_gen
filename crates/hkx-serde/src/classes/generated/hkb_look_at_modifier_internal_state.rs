//! A Rust structure that implements a serializer/deserializer corresponding to `hkbLookAtModifierInternalState`, a class defined in C++
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
/// -    size: 48
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbLookAtModifierInternalState<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbLookAtModifierInternalState"`: Name of this class.
    #[serde(default = "HkbLookAtModifierInternalState::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa14caba6`: Unique value of this class.
    #[serde(default = "HkbLookAtModifierInternalState::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbLookAtModifierInternalStateHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbLookAtModifierInternalStateHkParam<'a>>
}

impl HkbLookAtModifierInternalState<'_> {
    /// Return `"hkbLookAtModifierInternalState"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbLookAtModifierInternalState".into()
    }

    /// Return `"0xa14caba6"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa14caba6".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbLookAtModifierInternalStateHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"lookAtLastTargetWS"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lookAtLastTargetWS")]
    LookAtLastTargetWs(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"lookAtWeight"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lookAtWeight")]
    LookAtWeight(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"isTargetInsideLimitCone"`
    /// -   type: `hkBool`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isTargetInsideLimitCone")]
    IsTargetInsideLimitCone(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbLookAtModifierInternalStateHkParam<'de>, "@name",
    ("lookAtLastTargetWS" => LookAtLastTargetWs(cgmath::Vector4<f32>)),
    ("lookAtWeight" => LookAtWeight(f64)),
    ("isTargetInsideLimitCone" => IsTargetInsideLimitCone(bool)),
}
