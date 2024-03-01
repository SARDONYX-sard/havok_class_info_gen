//! A Rust structure that implements a serializer/deserializer corresponding to `hkpShapeCollection`, a class defined in C++
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
/// -    size: 24
/// -  vtable: true
/// -  parent: hkpShape/`666490a1`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpShapeCollection<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpShapeCollection"`: Name of this class.
    #[serde(default = "HkpShapeCollection::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xe8c3991d`: Unique value of this class.
    #[serde(default = "HkpShapeCollection::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpShapeCollectionHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpShapeCollectionHkParam<'a>>
}

impl HkpShapeCollection<'_> {
    /// Return `"hkpShapeCollection"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpShapeCollection".into()
    }

    /// Return `"0xe8c3991d"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xe8c3991d".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpShapeCollectionHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"disableWelding"`
    /// -   type: `hkBool`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "disableWelding")]
    DisableWelding(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"collectionType"`
    /// -   type: `enum CollectionType`
    /// - offset: 21
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collectionType")]
    CollectionType(CollectionType),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpShapeCollectionHkParam<'de>, "@name",
    ("disableWelding" => DisableWelding(bool)),
    ("collectionType" => CollectionType(CollectionType)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CollectionType {
    #[serde(rename = "COLLECTION_LIST")]
    CollectionList = 0,
    #[serde(rename = "COLLECTION_EXTENDED_MESH")]
    CollectionExtendedMesh = 1,
    #[serde(rename = "COLLECTION_TRISAMPLED_HEIGHTFIELD")]
    CollectionTrisampledHeightfield = 2,
    #[serde(rename = "COLLECTION_USER")]
    CollectionUser = 3,
    #[serde(rename = "COLLECTION_SIMPLE_MESH")]
    CollectionSimpleMesh = 4,
    #[serde(rename = "COLLECTION_MESH_SHAPE")]
    CollectionMeshShape = 5,
    #[serde(rename = "COLLECTION_COMPRESSED_MESH")]
    CollectionCompressedMesh = 6,
    #[serde(rename = "COLLECTION_MAX")]
    CollectionMax = 7,
}
