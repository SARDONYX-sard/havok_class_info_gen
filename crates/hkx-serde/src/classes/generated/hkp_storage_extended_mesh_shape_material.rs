//! A Rust structure that implements a serializer/deserializer corresponding to `hkpStorageExtendedMeshShapeMaterial`, a class defined in C++
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
/// -    size: 12
/// -  vtable: false
/// -  parent: hkpMeshMaterial/`886cde0c`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpStorageExtendedMeshShapeMaterial<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpStorageExtendedMeshShapeMaterial"`: The original C++ class name.
    #[serde(default = "HkpStorageExtendedMeshShapeMaterial::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x2ca3e906`: Unique value of this class.
    #[serde(default = "HkpStorageExtendedMeshShapeMaterial::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpStorageExtendedMeshShapeMaterialHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpStorageExtendedMeshShapeMaterialHkParam<'a>>
}

impl HkpStorageExtendedMeshShapeMaterial<'_> {
    /// Return `"hkpStorageExtendedMeshShapeMaterial"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpStorageExtendedMeshShapeMaterial".into()
    }

    /// Return `"0x2ca3e906"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x2ca3e906".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpStorageExtendedMeshShapeMaterialHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"restitution"`
    /// -   type: `hkHalf`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "restitution")]
    Restitution(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"friction"`
    /// -   type: `hkHalf`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "friction")]
    Friction(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<u64>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpStorageExtendedMeshShapeMaterialHkParam<'de>, "@name",
    ("restitution" => Restitution(Primitive<f32>)),
    ("friction" => Friction(Primitive<f32>)),
    ("userData" => UserData(Primitive<u64>)),
}
