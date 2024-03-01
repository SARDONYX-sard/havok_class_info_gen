//! A Rust structure that implements a serializer/deserializer corresponding to `hkMotionState`, a class defined in C++
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
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMotionState<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMotionState"`: Name of this class.
    #[serde(default = "HkMotionState::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x5797386e`: Unique value of this class.
    #[serde(default = "HkMotionState::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMotionStateHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMotionStateHkParam<'a>>
}

impl HkMotionState<'_> {
    /// Return `"hkMotionState"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkMotionState".into()
    }

    /// Return `"0x5797386e"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x5797386e".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMotionStateHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"transform"`
    /// -   type: `hkTransform`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transform")]
    Transform(cgmath::Matrix4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"sweptTransform"`
    /// -   type: `struct hkSweptTransform`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sweptTransform")]
    SweptTransform(HkSweptTransform),
    /// # Information on fields in the original C++ class
    /// -   name:`"deltaAngle"`
    /// -   type: `hkVector4`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deltaAngle")]
    DeltaAngle(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"objectRadius"`
    /// -   type: `hkReal`
    /// - offset: 160
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "objectRadius")]
    ObjectRadius(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"linearDamping"`
    /// -   type: `hkHalf`
    /// - offset: 164
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "linearDamping")]
    LinearDamping(f32),
    /// # Information on fields in the original C++ class
    /// -   name:`"angularDamping"`
    /// -   type: `hkHalf`
    /// - offset: 166
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "angularDamping")]
    AngularDamping(f32),
    /// # Information on fields in the original C++ class
    /// -   name:`"timeFactor"`
    /// -   type: `hkHalf`
    /// - offset: 168
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timeFactor")]
    TimeFactor(f32),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxLinearVelocity"`
    /// -   type: `hkUint8`
    /// - offset: 170
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxLinearVelocity")]
    MaxLinearVelocity(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxAngularVelocity"`
    /// -   type: `hkUint8`
    /// - offset: 171
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxAngularVelocity")]
    MaxAngularVelocity(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"deactivationClass"`
    /// -   type: `hkUint8`
    /// - offset: 172
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivationClass")]
    DeactivationClass(u8),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMotionStateHkParam<'de>, "@name",
    ("transform" => Transform(cgmath::Matrix4<f32>)),
    ("sweptTransform" => SweptTransform(HkSweptTransform)),
    ("deltaAngle" => DeltaAngle(cgmath::Vector4<f32>)),
    ("objectRadius" => ObjectRadius(f64)),
    ("linearDamping" => LinearDamping(f32)),
    ("angularDamping" => AngularDamping(f32)),
    ("timeFactor" => TimeFactor(f32)),
    ("maxLinearVelocity" => MaxLinearVelocity(u8)),
    ("maxAngularVelocity" => MaxAngularVelocity(u8)),
    ("deactivationClass" => DeactivationClass(u8)),
}
