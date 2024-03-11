//! A Rust structure that implements a serializer/deserializer corresponding to `hkMeshTexture`, a class defined in C++
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
/// -    size: 8
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMeshTexture<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMeshTexture"`: The original C++ class name.
    #[serde(default = "HkMeshTexture::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xc9887918`: Unique value of this class.
    #[serde(default = "HkMeshTexture::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMeshTextureHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMeshTextureHkParam<'a>>
}

impl HkMeshTexture<'_> {
    /// Return `"hkMeshTexture"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkMeshTexture".into()
    }

    /// Return `"0xc9887918"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc9887918".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMeshTextureHkParam<'a> {
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMeshTextureHkParam<'de>, "@name",
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "Unknown")]
    Unknown = 0,
    #[serde(rename = "PNG")]
    Png = 1,
    #[serde(rename = "TGA")]
    Tga = 2,
    #[serde(rename = "BMP")]
    Bmp = 3,
    #[serde(rename = "DDS")]
    Dds = 4,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FilterMode {
    #[serde(rename = "POINT")]
    Point = 0,
    #[serde(rename = "LINEAR")]
    Linear = 1,
    #[serde(rename = "ANISOTROPIC")]
    Anisotropic = 2,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TextureUsageType {
    #[serde(rename = "UNKNOWN")]
    Unknown = 0,
    #[serde(rename = "DIFFUSE")]
    Diffuse = 1,
    #[serde(rename = "REFLECTION")]
    Reflection = 2,
    #[serde(rename = "BUMP")]
    Bump = 3,
    #[serde(rename = "NORMAL")]
    Normal = 4,
    #[serde(rename = "DISPLACEMENT")]
    Displacement = 5,
    #[serde(rename = "SPECULAR")]
    Specular = 6,
    #[serde(rename = "SPECULARANDGLOSS")]
    Specularandgloss = 7,
    #[serde(rename = "OPACITY")]
    Opacity = 8,
    #[serde(rename = "EMISSIVE")]
    Emissive = 9,
    #[serde(rename = "REFRACTION")]
    Refraction = 10,
    #[serde(rename = "GLOSS")]
    Gloss = 11,
    #[serde(rename = "NOTEXPORTED")]
    Notexported = 12,
}
