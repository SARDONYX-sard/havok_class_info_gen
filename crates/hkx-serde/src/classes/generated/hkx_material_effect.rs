//! A Rust structure that implements a serializer/deserializer corresponding to `hkxMaterialEffect`, a class defined in C++
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
/// -    size: 28
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxMaterialEffect<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxMaterialEffect"`: Name of this class.
    #[serde(default = "HkxMaterialEffect::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x1d39f925`: Unique value of this class.
    #[serde(default = "HkxMaterialEffect::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxMaterialEffectHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxMaterialEffectHkParam<'a>>
}

impl HkxMaterialEffect<'_> {
    /// Return `"hkxMaterialEffect"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkxMaterialEffect".into()
    }

    /// Return `"0x1d39f925"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x1d39f925".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxMaterialEffectHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum EffectType`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(EffectType),
    /// # Information on fields in the original C++ class
    /// -   name:`"data"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Vec<u8>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxMaterialEffectHkParam<'de>, "@name",
    ("name" => Name(String)),
    ("type" => Type(EffectType)),
    ("data" => Data(Vec<u8>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EffectType {
    #[serde(rename = "EFFECT_TYPE_INVALID")]
    EffectTypeInvalid = 0,
    #[serde(rename = "EFFECT_TYPE_UNKNOWN")]
    EffectTypeUnknown = 1,
    #[serde(rename = "EFFECT_TYPE_HLSL_FX_INLINE")]
    EffectTypeHlslFxInline = 2,
    #[serde(rename = "EFFECT_TYPE_CG_FX_INLINE")]
    EffectTypeCgFxInline = 3,
    #[serde(rename = "EFFECT_TYPE_HLSL_FX_FILENAME")]
    EffectTypeHlslFxFilename = 4,
    #[serde(rename = "EFFECT_TYPE_CG_FX_FILENAME")]
    EffectTypeCgFxFilename = 5,
    #[serde(rename = "EFFECT_TYPE_MAX_ID")]
    EffectTypeMaxId = 6,
}
