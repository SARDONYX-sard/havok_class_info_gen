//! A Rust structure that implements a serializer/deserializer corresponding to `hkMemoryMeshTexture`, a class defined in C++
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
/// -    size: 32
/// -  vtable: true
/// -  parent: hkMeshTexture/`c9887918`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMemoryMeshTexture<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMemoryMeshTexture"`: The original C++ class name.
    #[serde(default = "HkMemoryMeshTexture::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x2db6577c`: Unique value of this class.
    #[serde(default = "HkMemoryMeshTexture::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMemoryMeshTextureHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMemoryMeshTextureHkParam<'a>>
}

impl HkMemoryMeshTexture<'_> {
    /// Return `"hkMemoryMeshTexture"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkMemoryMeshTexture".into()
    }

    /// Return `"0x2db6577c"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x2db6577c".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMemoryMeshTextureHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"filename"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "filename")]
    Filename(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"data"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Vec<Primitive<u8>>),
    /// # Field information in the original C++ class
    /// -   name:`"format"`
    /// -   type: `enum Format`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "format")]
    Format(Format),
    /// # Field information in the original C++ class
    /// -   name:`"hasMipMaps"`
    /// -   type: `hkBool`
    /// - offset: 25
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hasMipMaps")]
    HasMipMaps(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"filterMode"`
    /// -   type: `enum FilterMode`
    /// - offset: 26
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "filterMode")]
    FilterMode(FilterMode),
    /// # Field information in the original C++ class
    /// -   name:`"usageHint"`
    /// -   type: `enum TextureUsageType`
    /// - offset: 27
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "usageHint")]
    UsageHint(TextureUsageType),
    /// # Field information in the original C++ class
    /// -   name:`"textureCoordChannel"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "textureCoordChannel")]
    TextureCoordChannel(Primitive<i32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryMeshTextureHkParam<'de>, "@name",
    ("filename" => Filename(Primitive<Cow<'a, str>>)),
    ("data" => Data(Vec<Primitive<u8>>)),
    ("format" => Format(Format)),
    ("hasMipMaps" => HasMipMaps(Primitive<bool>)),
    ("filterMode" => FilterMode(FilterMode)),
    ("usageHint" => UsageHint(TextureUsageType)),
    ("textureCoordChannel" => TextureCoordChannel(Primitive<i32>)),
}
