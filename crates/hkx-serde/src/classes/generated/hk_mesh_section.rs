//! A Rust structure that implements a serializer/deserializer corresponding to `hkMeshSection`, a class defined in C++
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
/// -    size: 40
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMeshSection<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMeshSection"`: Name of this class.
    #[serde(default = "HkMeshSection::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x1893c365`: Unique value of this class.
    #[serde(default = "HkMeshSection::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMeshSectionHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMeshSectionHkParam<'a>>
}

impl HkMeshSection<'_> {
    /// Return `"hkMeshSection"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkMeshSection".into()
    }

    /// Return `"0x1893c365"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x1893c365".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMeshSectionHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"primitiveType"`
    /// -   type: `enum PrimitiveType`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "primitiveType")]
    PrimitiveType(PrimitiveType),
    /// # Information on fields in the original C++ class
    /// -   name:`"numPrimitives"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numPrimitives")]
    NumPrimitives(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"numIndices"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numIndices")]
    NumIndices(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"vertexStartIndex"`
    /// -   type: `hkInt32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexStartIndex")]
    VertexStartIndex(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"transformIndex"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformIndex")]
    TransformIndex(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"indexType"`
    /// -   type: `enum MeshSectionIndexType`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexType")]
    IndexType(MeshSectionIndexType),
    /// # Information on fields in the original C++ class
    /// -   name:`"indices"`
    /// -   type: `void*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "indices", skip_serializing)]
    Indices(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"vertexBuffer"`
    /// -   type: `struct hkMeshVertexBuffer*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBuffer")]
    VertexBuffer(Box<HkMeshVertexBuffer>),
    /// # Information on fields in the original C++ class
    /// -   name:`"material"`
    /// -   type: `struct hkMeshMaterial*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "material")]
    Material(Box<HkMeshMaterial>),
    /// # Information on fields in the original C++ class
    /// -   name:`"sectionIndex"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sectionIndex")]
    SectionIndex(i32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMeshSectionHkParam<'de>, "@name",
    ("primitiveType" => PrimitiveType(PrimitiveType)),
    ("numPrimitives" => NumPrimitives(i32)),
    ("numIndices" => NumIndices(i32)),
    ("vertexStartIndex" => VertexStartIndex(i32)),
    ("transformIndex" => TransformIndex(i32)),
    ("indexType" => IndexType(MeshSectionIndexType)),
    ("indices" => Indices(())),
    ("vertexBuffer" => VertexBuffer(Box<HkMeshVertexBuffer>)),
    ("material" => Material(Box<HkMeshMaterial>)),
    ("sectionIndex" => SectionIndex(i32)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MeshSectionIndexType {
    #[serde(rename = "INDEX_TYPE_NONE")]
    IndexTypeNone = 0,
    #[serde(rename = "INDEX_TYPE_UINT16")]
    IndexTypeUint16 = 1,
    #[serde(rename = "INDEX_TYPE_UINT32")]
    IndexTypeUint32 = 2,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PrimitiveType {
    #[serde(rename = "PRIMITIVE_TYPE_UNKNOWN")]
    PrimitiveTypeUnknown = 0,
    #[serde(rename = "PRIMITIVE_TYPE_POINT_LIST")]
    PrimitiveTypePointList = 1,
    #[serde(rename = "PRIMITIVE_TYPE_LINE_LIST")]
    PrimitiveTypeLineList = 2,
    #[serde(rename = "PRIMITIVE_TYPE_TRIANGLE_LIST")]
    PrimitiveTypeTriangleList = 3,
    #[serde(rename = "PRIMITIVE_TYPE_TRIANGLE_STRIP")]
    PrimitiveTypeTriangleStrip = 4,
}
