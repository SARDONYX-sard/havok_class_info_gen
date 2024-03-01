//! A Rust structure that implements a serializer/deserializer corresponding to `hkpStorageMeshShapeSubpartStorage`, a class defined in C++
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
/// -    size: 80
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpStorageMeshShapeSubpartStorage<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpStorageMeshShapeSubpartStorage"`: Name of this class.
    #[serde(default = "HkpStorageMeshShapeSubpartStorage::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xbf27438`: Unique value of this class.
    #[serde(default = "HkpStorageMeshShapeSubpartStorage::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpStorageMeshShapeSubpartStorageHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpStorageMeshShapeSubpartStorageHkParam<'a>>
}

impl HkpStorageMeshShapeSubpartStorage<'_> {
    /// Return `"hkpStorageMeshShapeSubpartStorage"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpStorageMeshShapeSubpartStorage".into()
    }

    /// Return `"0xbf27438"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xbf27438".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpStorageMeshShapeSubpartStorageHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"vertices"`
    /// -   type: `hkArray&lt;hkReal&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertices")]
    Vertices(Vec<f64>),
    /// # Information on fields in the original C++ class
    /// -   name:`"indices16"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices16")]
    Indices16(Vec<u16>),
    /// # Information on fields in the original C++ class
    /// -   name:`"indices32"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices32")]
    Indices32(Vec<u32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"materialIndices"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndices")]
    MaterialIndices(Vec<u8>),
    /// # Information on fields in the original C++ class
    /// -   name:`"materials"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials")]
    Materials(Vec<u32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"materialIndices16"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndices16")]
    MaterialIndices16(Vec<u16>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpStorageMeshShapeSubpartStorageHkParam<'de>, "@name",
    ("vertices" => Vertices(Vec<f64>)),
    ("indices16" => Indices16(Vec<u16>)),
    ("indices32" => Indices32(Vec<u32>)),
    ("materialIndices" => MaterialIndices(Vec<u8>)),
    ("materials" => Materials(Vec<u32>)),
    ("materialIndices16" => MaterialIndices16(Vec<u16>)),
}
