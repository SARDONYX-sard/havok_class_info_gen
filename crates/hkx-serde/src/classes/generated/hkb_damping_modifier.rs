//! A Rust structure that implements a serializer/deserializer corresponding to `hkbDampingModifier`, a class defined in C++
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
/// -    size: 160
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbDampingModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbDampingModifier"`: Name of this class.
    #[serde(default = "HkbDampingModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x9a040f03`: Unique value of this class.
    #[serde(default = "HkbDampingModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbDampingModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbDampingModifierHkParam<'a>>
}

impl HkbDampingModifier<'_> {
    /// Return `"hkbDampingModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbDampingModifier".into()
    }

    /// Return `"0x9a040f03"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x9a040f03".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbDampingModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"kP"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kP")]
    KP(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"kI"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kI")]
    KI(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"kD"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kD")]
    KD(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"enableScalarDamping"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableScalarDamping")]
    EnableScalarDamping(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"enableVectorDamping"`
    /// -   type: `hkBool`
    /// - offset: 57
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableVectorDamping")]
    EnableVectorDamping(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"rawValue"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rawValue")]
    RawValue(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"dampedValue"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dampedValue")]
    DampedValue(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"rawVector"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rawVector")]
    RawVector(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"dampedVector"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dampedVector")]
    DampedVector(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"vecErrorSum"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vecErrorSum")]
    VecErrorSum(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"vecPreviousError"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vecPreviousError")]
    VecPreviousError(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"errorSum"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "errorSum")]
    ErrorSum(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"previousError"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previousError")]
    PreviousError(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbDampingModifierHkParam<'de>, "@name",
    ("kP" => KP(f64)),
    ("kI" => KI(f64)),
    ("kD" => KD(f64)),
    ("enableScalarDamping" => EnableScalarDamping(bool)),
    ("enableVectorDamping" => EnableVectorDamping(bool)),
    ("rawValue" => RawValue(f64)),
    ("dampedValue" => DampedValue(f64)),
    ("rawVector" => RawVector(cgmath::Vector4<f32>)),
    ("dampedVector" => DampedVector(cgmath::Vector4<f32>)),
    ("vecErrorSum" => VecErrorSum(cgmath::Vector4<f32>)),
    ("vecPreviousError" => VecPreviousError(cgmath::Vector4<f32>)),
    ("errorSum" => ErrorSum(f64)),
    ("previousError" => PreviousError(f64)),
}
