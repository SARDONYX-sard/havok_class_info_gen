//! A Rust structure that implements a serializer/deserializer corresponding to `hkMeshSectionCinfo`, a class defined in C++
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
/// -    size: 32
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMeshSectionCinfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMeshSectionCinfo"`: Name of this class.
    #[serde(default = "HkMeshSectionCinfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x6075f3ff`: Unique value of this class.
    #[serde(default = "HkMeshSectionCinfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMeshSectionCinfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMeshSectionCinfoHkParam<'a>>
}

impl HkMeshSectionCinfo<'_> {
    /// Return `"hkMeshSectionCinfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkMeshSectionCinfo".into()
    }

    /// Return `"0x6075f3ff"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x6075f3ff".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMeshSectionCinfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"vertexBuffer"`
    /// -   type: `struct hkMeshVertexBuffer*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBuffer")]
    VertexBuffer(Box<HkMeshVertexBuffer>),
    /// # Information on fields in the original C++ class
    /// -   name:`"material"`
    /// -   type: `struct hkMeshMaterial*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "material")]
    Material(Box<HkMeshMaterial>),
    /// # Information on fields in the original C++ class
    /// -   name:`"primitiveType"`
    /// -   type: `enum PrimitiveType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "primitiveType")]
    PrimitiveType(PrimitiveType),
    /// # Information on fields in the original C++ class
    /// -   name:`"numPrimitives"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numPrimitives")]
    NumPrimitives(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"indexType"`
    /// -   type: `enum MeshSectionIndexType`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexType")]
    IndexType(MeshSectionIndexType),
    /// # Information on fields in the original C++ class
    /// -   name:`"indices"`
    /// -   type: `void*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "indices", skip_serializing)]
    Indices(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"vertexStartIndex"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexStartIndex")]
    VertexStartIndex(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"transformIndex"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformIndex")]
    TransformIndex(i32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMeshSectionCinfoHkParam<'de>, "@name",
    ("vertexBuffer" => VertexBuffer(Box<HkMeshVertexBuffer>)),
    ("material" => Material(Box<HkMeshMaterial>)),
    ("primitiveType" => PrimitiveType(PrimitiveType)),
    ("numPrimitives" => NumPrimitives(i32)),
    ("indexType" => IndexType(MeshSectionIndexType)),
    ("indices" => Indices(())),
    ("vertexStartIndex" => VertexStartIndex(i32)),
    ("transformIndex" => TransformIndex(i32)),
}
