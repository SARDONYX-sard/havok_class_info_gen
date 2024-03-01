//! A Rust structure that implements a serializer/deserializer corresponding to `hkbComputeDirectionModifier`, a class defined in C++
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
/// -    size: 112
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbComputeDirectionModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbComputeDirectionModifier"`: Name of this class.
    #[serde(default = "HkbComputeDirectionModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xdf358bd3`: Unique value of this class.
    #[serde(default = "HkbComputeDirectionModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbComputeDirectionModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbComputeDirectionModifierHkParam<'a>>
}

impl HkbComputeDirectionModifier<'_> {
    /// Return `"hkbComputeDirectionModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbComputeDirectionModifier".into()
    }

    /// Return `"0xdf358bd3"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xdf358bd3".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbComputeDirectionModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"pointIn"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pointIn")]
    PointIn(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"pointOut"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pointOut")]
    PointOut(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"groundAngleOut"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "groundAngleOut")]
    GroundAngleOut(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"upAngleOut"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "upAngleOut")]
    UpAngleOut(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"verticalOffset"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "verticalOffset")]
    VerticalOffset(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"reverseGroundAngle"`
    /// -   type: `hkBool`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "reverseGroundAngle")]
    ReverseGroundAngle(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"reverseUpAngle"`
    /// -   type: `hkBool`
    /// - offset: 93
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "reverseUpAngle")]
    ReverseUpAngle(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"projectPoint"`
    /// -   type: `hkBool`
    /// - offset: 94
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "projectPoint")]
    ProjectPoint(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"normalizePoint"`
    /// -   type: `hkBool`
    /// - offset: 95
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "normalizePoint")]
    NormalizePoint(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"computeOnlyOnce"`
    /// -   type: `hkBool`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "computeOnlyOnce")]
    ComputeOnlyOnce(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"computedOutput"`
    /// -   type: `hkBool`
    /// - offset: 97
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "computedOutput")]
    ComputedOutput(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbComputeDirectionModifierHkParam<'de>, "@name",
    ("pointIn" => PointIn(cgmath::Vector4<f32>)),
    ("pointOut" => PointOut(cgmath::Vector4<f32>)),
    ("groundAngleOut" => GroundAngleOut(f64)),
    ("upAngleOut" => UpAngleOut(f64)),
    ("verticalOffset" => VerticalOffset(f64)),
    ("reverseGroundAngle" => ReverseGroundAngle(bool)),
    ("reverseUpAngle" => ReverseUpAngle(bool)),
    ("projectPoint" => ProjectPoint(bool)),
    ("normalizePoint" => NormalizePoint(bool)),
    ("computeOnlyOnce" => ComputeOnlyOnce(bool)),
    ("computedOutput" => ComputedOutput(bool)),
}
