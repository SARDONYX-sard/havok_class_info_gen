//! A Rust structure that implements a serializer/deserializer corresponding to `hkpCompressedMeshShapeBigTriangle`, a class defined in C++
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
/// -    size: 16
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpCompressedMeshShapeBigTriangle<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpCompressedMeshShapeBigTriangle"`: The original C++ class name.
    #[serde(default = "HkpCompressedMeshShapeBigTriangle::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xcbfc95a4`: Unique value of this class.
    #[serde(default = "HkpCompressedMeshShapeBigTriangle::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpCompressedMeshShapeBigTriangleHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpCompressedMeshShapeBigTriangleHkParam<'a>>
}

impl HkpCompressedMeshShapeBigTriangle<'_> {
    /// Return `"hkpCompressedMeshShapeBigTriangle"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpCompressedMeshShapeBigTriangle".into()
    }

    /// Return `"0xcbfc95a4"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xcbfc95a4".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCompressedMeshShapeBigTriangleHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"a"`
    /// -   type: `hkUint16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "a")]
    A(Primitive<u16>),
    /// # Field information in the original C++ class
    /// -   name:`"b"`
    /// -   type: `hkUint16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "b")]
    B(Primitive<u16>),
    /// # Field information in the original C++ class
    /// -   name:`"c"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "c")]
    C(Primitive<u16>),
    /// # Field information in the original C++ class
    /// -   name:`"material"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "material")]
    Material(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"weldingInfo"`
    /// -   type: `hkUint16`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingInfo")]
    WeldingInfo(Primitive<u16>),
    /// # Field information in the original C++ class
    /// -   name:`"transformIndex"`
    /// -   type: `hkUint16`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformIndex")]
    TransformIndex(Primitive<u16>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpCompressedMeshShapeBigTriangleHkParam<'de>, "@name",
    ("a" => A(Primitive<u16>)),
    ("b" => B(Primitive<u16>)),
    ("c" => C(Primitive<u16>)),
    ("material" => Material(Primitive<u32>)),
    ("weldingInfo" => WeldingInfo(Primitive<u16>)),
    ("transformIndex" => TransformIndex(Primitive<u16>)),
}
