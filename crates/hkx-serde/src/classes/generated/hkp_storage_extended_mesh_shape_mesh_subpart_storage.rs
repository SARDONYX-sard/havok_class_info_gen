//! A Rust structure that implements a serializer/deserializer corresponding to `hkpStorageExtendedMeshShapeMeshSubpartStorage`, a class defined in C++
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
/// -    size: 104
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 3
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpStorageExtendedMeshShapeMeshSubpartStorage<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpStorageExtendedMeshShapeMeshSubpartStorage"`: The original C++ class name.
    #[serde(default = "HkpStorageExtendedMeshShapeMeshSubpartStorage::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x5aad4de6`: Unique value of this class.
    #[serde(default = "HkpStorageExtendedMeshShapeMeshSubpartStorage::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpStorageExtendedMeshShapeMeshSubpartStorageHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpStorageExtendedMeshShapeMeshSubpartStorageHkParam<'a>>
}

impl HkpStorageExtendedMeshShapeMeshSubpartStorage<'_> {
    /// Return `"hkpStorageExtendedMeshShapeMeshSubpartStorage"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpStorageExtendedMeshShapeMeshSubpartStorage".into()
    }

    /// Return `"0x5aad4de6"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x5aad4de6".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpStorageExtendedMeshShapeMeshSubpartStorageHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"vertices"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertices")]
    Vertices(Vec<Vector4<f32>>),
    /// # Field information in the original C++ class
    /// -   name:`"indices8"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices8")]
    Indices8(Vec<Primitive<u8>>),
    /// # Field information in the original C++ class
    /// -   name:`"indices16"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices16")]
    Indices16(Vec<Primitive<u16>>),
    /// # Field information in the original C++ class
    /// -   name:`"indices32"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices32")]
    Indices32(Vec<Primitive<u32>>),
    /// # Field information in the original C++ class
    /// -   name:`"materialIndices"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndices")]
    MaterialIndices(Vec<Primitive<u8>>),
    /// # Field information in the original C++ class
    /// -   name:`"materials"`
    /// -   type: `hkArray&lt;struct hkpStorageExtendedMeshShapeMaterial&gt;`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials")]
    Materials(Vec<HkpStorageExtendedMeshShapeMaterial>),
    /// # Field information in the original C++ class
    /// -   name:`"namedMaterials"`
    /// -   type: `hkArray&lt;struct hkpNamedMeshMaterial&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "namedMaterials")]
    NamedMaterials(Vec<HkpNamedMeshMaterial>),
    /// # Field information in the original C++ class
    /// -   name:`"materialIndices16"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndices16")]
    MaterialIndices16(Vec<Primitive<u16>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpStorageExtendedMeshShapeMeshSubpartStorageHkParam<'de>, "@name",
    ("vertices" => Vertices(Vec<Vector4<f32>>)),
    ("indices8" => Indices8(Vec<Primitive<u8>>)),
    ("indices16" => Indices16(Vec<Primitive<u16>>)),
    ("indices32" => Indices32(Vec<Primitive<u32>>)),
    ("materialIndices" => MaterialIndices(Vec<Primitive<u8>>)),
    ("materials" => Materials(Vec<HkpStorageExtendedMeshShapeMaterial>)),
    ("namedMaterials" => NamedMaterials(Vec<HkpNamedMeshMaterial>)),
    ("materialIndices16" => MaterialIndices16(Vec<Primitive<u16>>)),
}
