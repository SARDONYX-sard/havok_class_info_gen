//! A Rust structure that implements a serializer/deserializer corresponding to `hkpMeshShapeSubpart`, a class defined in C++
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
/// -    size: 56
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpMeshShapeSubpart<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpMeshShapeSubpart"`: Name of this class.
    #[serde(default = "HkpMeshShapeSubpart::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x27336e5d`: Unique value of this class.
    #[serde(default = "HkpMeshShapeSubpart::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpMeshShapeSubpartHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpMeshShapeSubpartHkParam<'a>>
}

impl HkpMeshShapeSubpart<'_> {
    /// Return `"hkpMeshShapeSubpart"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpMeshShapeSubpart".into()
    }

    /// Return `"0x27336e5d"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x27336e5d".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpMeshShapeSubpartHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"vertexBase"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "vertexBase", skip_serializing)]
    VertexBase(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"vertexStriding"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexStriding")]
    VertexStriding(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"numVertices"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numVertices")]
    NumVertices(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"indexBase"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "indexBase", skip_serializing)]
    IndexBase(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"stridingType"`
    /// -   type: `enum MeshShapeIndexStridingType`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stridingType")]
    StridingType(MeshShapeIndexStridingType),
    /// # Information on fields in the original C++ class
    /// -   name:`"materialIndexStridingType"`
    /// -   type: `enum MeshShapeMaterialIndexStridingType`
    /// - offset: 17
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndexStridingType")]
    MaterialIndexStridingType(MeshShapeMaterialIndexStridingType),
    /// # Information on fields in the original C++ class
    /// -   name:`"indexStriding"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexStriding")]
    IndexStriding(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"flipAlternateTriangles"`
    /// -   type: `hkInt32`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flipAlternateTriangles")]
    FlipAlternateTriangles(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"numTriangles"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numTriangles")]
    NumTriangles(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"materialIndexBase"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "materialIndexBase", skip_serializing)]
    MaterialIndexBase(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"materialIndexStriding"`
    /// -   type: `hkInt32`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndexStriding")]
    MaterialIndexStriding(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"materialBase"`
    /// -   type: `void*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "materialBase", skip_serializing)]
    MaterialBase(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"materialStriding"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialStriding")]
    MaterialStriding(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"numMaterials"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numMaterials")]
    NumMaterials(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"triangleOffset"`
    /// -   type: `hkInt32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triangleOffset")]
    TriangleOffset(i32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpMeshShapeSubpartHkParam<'de>, "@name",
    ("vertexBase" => VertexBase(())),
    ("vertexStriding" => VertexStriding(i32)),
    ("numVertices" => NumVertices(i32)),
    ("indexBase" => IndexBase(())),
    ("stridingType" => StridingType(MeshShapeIndexStridingType)),
    ("materialIndexStridingType" => MaterialIndexStridingType(MeshShapeMaterialIndexStridingType)),
    ("indexStriding" => IndexStriding(i32)),
    ("flipAlternateTriangles" => FlipAlternateTriangles(i32)),
    ("numTriangles" => NumTriangles(i32)),
    ("materialIndexBase" => MaterialIndexBase(())),
    ("materialIndexStriding" => MaterialIndexStriding(i32)),
    ("materialBase" => MaterialBase(())),
    ("materialStriding" => MaterialStriding(i32)),
    ("numMaterials" => NumMaterials(i32)),
    ("triangleOffset" => TriangleOffset(i32)),
}
