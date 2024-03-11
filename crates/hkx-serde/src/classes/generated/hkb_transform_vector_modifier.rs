//! A Rust structure that implements a serializer/deserializer corresponding to `hkbTransformVectorModifier`, a class defined in C++
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
/// -    size: 128
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbTransformVectorModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbTransformVectorModifier"`: The original C++ class name.
    #[serde(default = "HkbTransformVectorModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf93e0e24`: Unique value of this class.
    #[serde(default = "HkbTransformVectorModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbTransformVectorModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbTransformVectorModifierHkParam<'a>>
}

impl HkbTransformVectorModifier<'_> {
    /// Return `"hkbTransformVectorModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbTransformVectorModifier".into()
    }

    /// Return `"0xf93e0e24"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf93e0e24".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbTransformVectorModifierHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"rotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotation")]
    Rotation(Quaternion<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"translation"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translation")]
    Translation(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"vectorIn"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vectorIn")]
    VectorIn(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"vectorOut"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vectorOut")]
    VectorOut(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"rotateOnly"`
    /// -   type: `hkBool`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotateOnly")]
    RotateOnly(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"inverse"`
    /// -   type: `hkBool`
    /// - offset: 113
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "inverse")]
    Inverse(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"computeOnActivate"`
    /// -   type: `hkBool`
    /// - offset: 114
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "computeOnActivate")]
    ComputeOnActivate(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"computeOnModify"`
    /// -   type: `hkBool`
    /// - offset: 115
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "computeOnModify")]
    ComputeOnModify(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbTransformVectorModifierHkParam<'de>, "@name",
    ("rotation" => Rotation(Quaternion<f32>)),
    ("translation" => Translation(Vector4<f32>)),
    ("vectorIn" => VectorIn(Vector4<f32>)),
    ("vectorOut" => VectorOut(Vector4<f32>)),
    ("rotateOnly" => RotateOnly(Primitive<bool>)),
    ("inverse" => Inverse(Primitive<bool>)),
    ("computeOnActivate" => ComputeOnActivate(Primitive<bool>)),
    ("computeOnModify" => ComputeOnModify(Primitive<bool>)),
}
