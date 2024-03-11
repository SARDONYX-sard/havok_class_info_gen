//! A Rust structure that implements a serializer/deserializer corresponding to `hkxMaterial`, a class defined in C++
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
/// -    size: 144
/// -  vtable: true
/// -  parent: hkxAttributeHolder/`7468cc44`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxMaterial<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxMaterial"`: The original C++ class name.
    #[serde(default = "HkxMaterial::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x2954537a`: Unique value of this class.
    #[serde(default = "HkxMaterial::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxMaterialHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxMaterialHkParam<'a>>
}

impl HkxMaterial<'_> {
    /// Return `"hkxMaterial"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkxMaterial".into()
    }

    /// Return `"0x2954537a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x2954537a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxMaterialHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"stages"`
    /// -   type: `hkArray&lt;struct hkxMaterialTextureStage&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stages")]
    Stages(Vec<HkxMaterialTextureStage>),
    /// # Field information in the original C++ class
    /// -   name:`"diffuseColor"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "diffuseColor")]
    DiffuseColor(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"ambientColor"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ambientColor")]
    AmbientColor(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"specularColor"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "specularColor")]
    SpecularColor(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"emissiveColor"`
    /// -   type: `hkVector4`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "emissiveColor")]
    EmissiveColor(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"subMaterials"`
    /// -   type: `hkArray&lt;hkxMaterial*&gt;`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "subMaterials")]
    SubMaterials(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"extraData"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extraData")]
    ExtraData(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"properties"`
    /// -   type: `hkArray&lt;struct hkxMaterialProperty&gt;`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "properties")]
    Properties(Vec<HkxMaterialProperty>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxMaterialHkParam<'de>, "@name",
    ("name" => Name(Primitive<Cow<'a, str>>)),
    ("stages" => Stages(Vec<HkxMaterialTextureStage>)),
    ("diffuseColor" => DiffuseColor(Vector4<f32>)),
    ("ambientColor" => AmbientColor(Vector4<f32>)),
    ("specularColor" => SpecularColor(Vector4<f32>)),
    ("emissiveColor" => EmissiveColor(Vector4<f32>)),
    ("subMaterials" => SubMaterials(Vec<Cow<'a, str>>)),
    ("extraData" => ExtraData(Cow<'a, str>)),
    ("properties" => Properties(Vec<HkxMaterialProperty>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TextureType {
    #[serde(rename = "TEX_UNKNOWN")]
    TexUnknown = 0,
    #[serde(rename = "TEX_DIFFUSE")]
    TexDiffuse = 1,
    #[serde(rename = "TEX_REFLECTION")]
    TexReflection = 2,
    #[serde(rename = "TEX_BUMP")]
    TexBump = 3,
    #[serde(rename = "TEX_NORMAL")]
    TexNormal = 4,
    #[serde(rename = "TEX_DISPLACEMENT")]
    TexDisplacement = 5,
    #[serde(rename = "TEX_SPECULAR")]
    TexSpecular = 6,
    #[serde(rename = "TEX_SPECULARANDGLOSS")]
    TexSpecularandgloss = 7,
    #[serde(rename = "TEX_OPACITY")]
    TexOpacity = 8,
    #[serde(rename = "TEX_EMISSIVE")]
    TexEmissive = 9,
    #[serde(rename = "TEX_REFRACTION")]
    TexRefraction = 10,
    #[serde(rename = "TEX_GLOSS")]
    TexGloss = 11,
    #[serde(rename = "TEX_NOTEXPORTED")]
    TexNotexported = 12,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PropertyKey {
    #[serde(rename = "PROPERTY_MTL_TYPE_BLEND")]
    PropertyMtlTypeBlend = 1,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE0")]
    PropertyMtlUvIdStage0 = 256,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE1")]
    PropertyMtlUvIdStage1 = 257,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE2")]
    PropertyMtlUvIdStage2 = 258,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE3")]
    PropertyMtlUvIdStage3 = 259,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE4")]
    PropertyMtlUvIdStage4 = 260,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE5")]
    PropertyMtlUvIdStage5 = 261,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE6")]
    PropertyMtlUvIdStage6 = 262,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE7")]
    PropertyMtlUvIdStage7 = 263,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE8")]
    PropertyMtlUvIdStage8 = 264,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE9")]
    PropertyMtlUvIdStage9 = 265,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE10")]
    PropertyMtlUvIdStage10 = 266,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE11")]
    PropertyMtlUvIdStage11 = 267,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE12")]
    PropertyMtlUvIdStage12 = 268,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE13")]
    PropertyMtlUvIdStage13 = 269,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE14")]
    PropertyMtlUvIdStage14 = 270,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE15")]
    PropertyMtlUvIdStage15 = 271,
    #[serde(rename = "PROPERTY_MTL_UV_ID_STAGE_MAX")]
    PropertyMtlUvIdStageMax = 272,
}
