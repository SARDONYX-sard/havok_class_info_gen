//! A Rust structure that implements a serializer/deserializer corresponding to `BSLookAtModifierBoneData`, a class defined in C++
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
/// -    size: 64
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsLookAtModifierBoneData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSLookAtModifierBoneData"`: The original C++ class name.
    #[serde(default = "BsLookAtModifierBoneData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x29efee59`: Unique value of this class.
    #[serde(default = "BsLookAtModifierBoneData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsLookAtModifierBoneDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsLookAtModifierBoneDataHkParam<'a>>
}

impl BsLookAtModifierBoneData<'_> {
    /// Return `"BSLookAtModifierBoneData"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BSLookAtModifierBoneData".into()
    }

    /// Return `"0x29efee59"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x29efee59".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsLookAtModifierBoneDataHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"index"`
    /// -   type: `hkInt16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "index")]
    Index(Primitive<i16>),
    /// # Field information in the original C++ class
    /// -   name:`"fwdAxisLS"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fwdAxisLS")]
    FwdAxisLs(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"limitAngleDegrees"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "limitAngleDegrees")]
    LimitAngleDegrees(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"onGain"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "onGain")]
    OnGain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"offGain"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offGain")]
    OffGain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"enabled"`
    /// -   type: `hkBool`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "enabled")]
    Enabled(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"currentFwdAxisLS"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "currentFwdAxisLS", skip_serializing)]
    CurrentFwdAxisLs(Vector4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsLookAtModifierBoneDataHkParam<'de>, "@name",
    ("index" => Index(Primitive<i16>)),
    ("fwdAxisLS" => FwdAxisLs(Vector4<f32>)),
    ("limitAngleDegrees" => LimitAngleDegrees(Primitive<f32>)),
    ("onGain" => OnGain(Primitive<f32>)),
    ("offGain" => OffGain(Primitive<f32>)),
    ("enabled" => Enabled(Primitive<bool>)),
    ("currentFwdAxisLS" => CurrentFwdAxisLs(Vector4<f32>)),
}
