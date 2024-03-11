//! A Rust structure that implements a serializer/deserializer corresponding to `hkbDampingModifier`, a class defined in C++
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

    /// `"hkbDampingModifier"`: The original C++ class name.
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
    /// Return `"hkbDampingModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbDampingModifier".into()
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
    /// # Field information in the original C++ class
    /// -   name:`"kP"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kP")]
    KP(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"kI"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kI")]
    KI(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"kD"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "kD")]
    KD(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"enableScalarDamping"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableScalarDamping")]
    EnableScalarDamping(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"enableVectorDamping"`
    /// -   type: `hkBool`
    /// - offset: 57
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enableVectorDamping")]
    EnableVectorDamping(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"rawValue"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rawValue")]
    RawValue(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"dampedValue"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dampedValue")]
    DampedValue(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"rawVector"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rawVector")]
    RawVector(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"dampedVector"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dampedVector")]
    DampedVector(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"vecErrorSum"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vecErrorSum")]
    VecErrorSum(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"vecPreviousError"`
    /// -   type: `hkVector4`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vecPreviousError")]
    VecPreviousError(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"errorSum"`
    /// -   type: `hkReal`
    /// - offset: 144
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "errorSum")]
    ErrorSum(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"previousError"`
    /// -   type: `hkReal`
    /// - offset: 148
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previousError")]
    PreviousError(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbDampingModifierHkParam<'de>, "@name",
    ("kP" => KP(Primitive<f32>)),
    ("kI" => KI(Primitive<f32>)),
    ("kD" => KD(Primitive<f32>)),
    ("enableScalarDamping" => EnableScalarDamping(Primitive<bool>)),
    ("enableVectorDamping" => EnableVectorDamping(Primitive<bool>)),
    ("rawValue" => RawValue(Primitive<f32>)),
    ("dampedValue" => DampedValue(Primitive<f32>)),
    ("rawVector" => RawVector(Vector4<f32>)),
    ("dampedVector" => DampedVector(Vector4<f32>)),
    ("vecErrorSum" => VecErrorSum(Vector4<f32>)),
    ("vecPreviousError" => VecPreviousError(Vector4<f32>)),
    ("errorSum" => ErrorSum(Primitive<f32>)),
    ("previousError" => PreviousError(Primitive<f32>)),
}
