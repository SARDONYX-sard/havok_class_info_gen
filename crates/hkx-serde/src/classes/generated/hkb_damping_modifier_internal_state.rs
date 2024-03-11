//! A Rust structure that implements a serializer/deserializer corresponding to `hkbDampingModifierInternalState`, a class defined in C++
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
/// -    size: 80
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbDampingModifierInternalState<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbDampingModifierInternalState"`: The original C++ class name.
    #[serde(default = "HkbDampingModifierInternalState::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x508d3b36`: Unique value of this class.
    #[serde(default = "HkbDampingModifierInternalState::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbDampingModifierInternalStateHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbDampingModifierInternalStateHkParam<'a>>
}

impl HkbDampingModifierInternalState<'_> {
    /// Return `"hkbDampingModifierInternalState"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbDampingModifierInternalState".into()
    }

    /// Return `"0x508d3b36"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x508d3b36".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbDampingModifierInternalStateHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"dampedVector"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dampedVector")]
    DampedVector(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"vecErrorSum"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vecErrorSum")]
    VecErrorSum(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"vecPreviousError"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vecPreviousError")]
    VecPreviousError(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"dampedValue"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dampedValue")]
    DampedValue(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"errorSum"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "errorSum")]
    ErrorSum(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"previousError"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previousError")]
    PreviousError(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbDampingModifierInternalStateHkParam<'de>, "@name",
    ("dampedVector" => DampedVector(Vector4<f32>)),
    ("vecErrorSum" => VecErrorSum(Vector4<f32>)),
    ("vecPreviousError" => VecPreviousError(Vector4<f32>)),
    ("dampedValue" => DampedValue(Primitive<f32>)),
    ("errorSum" => ErrorSum(Primitive<f32>)),
    ("previousError" => PreviousError(Primitive<f32>)),
}
