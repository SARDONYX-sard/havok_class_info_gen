//! A Rust structure that implements a serializer/deserializer corresponding to `hkxMaterialShader`, a class defined in C++
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
/// -    size: 40
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxMaterialShader<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxMaterialShader"`: Name of this class.
    #[serde(default = "HkxMaterialShader::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x28515eff`: Unique value of this class.
    #[serde(default = "HkxMaterialShader::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxMaterialShaderHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxMaterialShaderHkParam<'a>>
}

impl HkxMaterialShader<'_> {
    /// Return `"hkxMaterialShader"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkxMaterialShader".into()
    }

    /// Return `"0x28515eff"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x28515eff".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxMaterialShaderHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum ShaderType`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(ShaderType),
    /// # Information on fields in the original C++ class
    /// -   name:`"vertexEntryName"`
    /// -   type: `hkStringPtr`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexEntryName")]
    VertexEntryName(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"geomEntryName"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "geomEntryName")]
    GeomEntryName(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"pixelEntryName"`
    /// -   type: `hkStringPtr`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pixelEntryName")]
    PixelEntryName(String),
    /// # Information on fields in the original C++ class
    /// -   name:`"data"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Vec<u8>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxMaterialShaderHkParam<'de>, "@name",
    ("name" => Name(String)),
    ("type" => Type(ShaderType)),
    ("vertexEntryName" => VertexEntryName(String)),
    ("geomEntryName" => GeomEntryName(String)),
    ("pixelEntryName" => PixelEntryName(String)),
    ("data" => Data(Vec<u8>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ShaderType {
    #[serde(rename = "EFFECT_TYPE_INVALID")]
    EffectTypeInvalid = 0,
    #[serde(rename = "EFFECT_TYPE_UNKNOWN")]
    EffectTypeUnknown = 1,
    #[serde(rename = "EFFECT_TYPE_HLSL_INLINE")]
    EffectTypeHlslInline = 2,
    #[serde(rename = "EFFECT_TYPE_CG_INLINE")]
    EffectTypeCgInline = 3,
    #[serde(rename = "EFFECT_TYPE_HLSL_FILENAME")]
    EffectTypeHlslFilename = 4,
    #[serde(rename = "EFFECT_TYPE_CG_FILENAME")]
    EffectTypeCgFilename = 5,
    #[serde(rename = "EFFECT_TYPE_MAX_ID")]
    EffectTypeMaxId = 6,
}
