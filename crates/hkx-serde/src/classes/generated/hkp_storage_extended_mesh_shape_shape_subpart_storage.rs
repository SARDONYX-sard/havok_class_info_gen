//! A Rust structure that implements a serializer/deserializer corresponding to `hkpStorageExtendedMeshShapeShapeSubpartStorage`, a class defined in C++
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
/// -    size: 44
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpStorageExtendedMeshShapeShapeSubpartStorage<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpStorageExtendedMeshShapeShapeSubpartStorage"`: Name of this class.
    #[serde(default = "HkpStorageExtendedMeshShapeShapeSubpartStorage::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x3f7d804c`: Unique value of this class.
    #[serde(default = "HkpStorageExtendedMeshShapeShapeSubpartStorage::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpStorageExtendedMeshShapeShapeSubpartStorageHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpStorageExtendedMeshShapeShapeSubpartStorageHkParam<'a>>
}

impl HkpStorageExtendedMeshShapeShapeSubpartStorage<'_> {
    /// Return `"hkpStorageExtendedMeshShapeShapeSubpartStorage"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpStorageExtendedMeshShapeShapeSubpartStorage".into()
    }

    /// Return `"0x3f7d804c"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x3f7d804c".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpStorageExtendedMeshShapeShapeSubpartStorageHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"materialIndices"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndices")]
    MaterialIndices(Vec<u8>),
    /// # Information on fields in the original C++ class
    /// -   name:`"materials"`
    /// -   type: `hkArray&lt;struct hkpStorageExtendedMeshShapeMaterial&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials")]
    Materials(Vec<HkpStorageExtendedMeshShapeMaterial>),
    /// # Information on fields in the original C++ class
    /// -   name:`"materialIndices16"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndices16")]
    MaterialIndices16(Vec<u16>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpStorageExtendedMeshShapeShapeSubpartStorageHkParam<'de>, "@name",
    ("materialIndices" => MaterialIndices(Vec<u8>)),
    ("materials" => Materials(Vec<HkpStorageExtendedMeshShapeMaterial>)),
    ("materialIndices16" => MaterialIndices16(Vec<u16>)),
}