//! A Rust structure that implements a serializer/deserializer corresponding to `BSTweenerModifier`, a class defined in C++
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
/// -    size: 176
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsTweenerModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSTweenerModifier"`: Name of this class.
    #[serde(default = "BsTweenerModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xd2d9a04`: Unique value of this class.
    #[serde(default = "BsTweenerModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsTweenerModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsTweenerModifierHkParam<'a>>
}

impl BsTweenerModifier<'_> {
    /// Return `"BSTweenerModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BsTweenerModifier".into()
    }

    /// Return `"0xd2d9a04"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xd2d9a04".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsTweenerModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"tweenPosition"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tweenPosition")]
    TweenPosition(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"tweenRotation"`
    /// -   type: `hkBool`
    /// - offset: 45
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tweenRotation")]
    TweenRotation(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"useTweenDuration"`
    /// -   type: `hkBool`
    /// - offset: 46
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useTweenDuration")]
    UseTweenDuration(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"tweenDuration"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "tweenDuration")]
    TweenDuration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"targetPosition"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetPosition")]
    TargetPosition(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"targetRotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "targetRotation")]
    TargetRotation(cgmath::Quaternion<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "duration", skip_serializing)]
    Duration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"startTransform"`
    /// -   type: `hkQsTransform`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "startTransform", skip_serializing)]
    StartTransform(cgmath::Matrix4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"time"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "time", skip_serializing)]
    Time(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsTweenerModifierHkParam<'de>, "@name",
    ("tweenPosition" => TweenPosition(bool)),
    ("tweenRotation" => TweenRotation(bool)),
    ("useTweenDuration" => UseTweenDuration(bool)),
    ("tweenDuration" => TweenDuration(f64)),
    ("targetPosition" => TargetPosition(cgmath::Vector4<f32>)),
    ("targetRotation" => TargetRotation(cgmath::Quaternion<f32>)),
    ("duration" => Duration(f64)),
    ("startTransform" => StartTransform(cgmath::Matrix4<f32>)),
    ("time" => Time(f64)),
}
