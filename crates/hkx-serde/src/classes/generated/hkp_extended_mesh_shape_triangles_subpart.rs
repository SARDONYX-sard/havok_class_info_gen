//! A Rust structure that implements a serializer/deserializer corresponding to `hkpExtendedMeshShapeTrianglesSubpart`, a class defined in C++
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
/// -    size: 112
/// -  vtable: false
/// -  parent: hkpExtendedMeshShapeSubpart/`f4608207`(Non prefix hex signature)
/// - version: 3
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpExtendedMeshShapeTrianglesSubpart<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpExtendedMeshShapeTrianglesSubpart"`: Name of this class.
    #[serde(default = "HkpExtendedMeshShapeTrianglesSubpart::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x44c32df6`: Unique value of this class.
    #[serde(default = "HkpExtendedMeshShapeTrianglesSubpart::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpExtendedMeshShapeTrianglesSubpartHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpExtendedMeshShapeTrianglesSubpartHkParam<'a>>
}

impl HkpExtendedMeshShapeTrianglesSubpart<'_> {
    /// Return `"hkpExtendedMeshShapeTrianglesSubpart"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpExtendedMeshShapeTrianglesSubpart".into()
    }

    /// Return `"0x44c32df6"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x44c32df6".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpExtendedMeshShapeTrianglesSubpartHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"numTriangleShapes"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numTriangleShapes")]
    NumTriangleShapes(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"vertexBase"`
    /// -   type: `void*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "vertexBase", skip_serializing)]
    VertexBase(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"numVertices"`
    /// -   type: `hkInt32`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numVertices")]
    NumVertices(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"indexBase"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "indexBase", skip_serializing)]
    IndexBase(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"vertexStriding"`
    /// -   type: `hkUint16`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexStriding")]
    VertexStriding(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"triangleOffset"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triangleOffset")]
    TriangleOffset(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"indexStriding"`
    /// -   type: `hkUint16`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexStriding")]
    IndexStriding(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"stridingType"`
    /// -   type: `enum IndexStridingType`
    /// - offset: 46
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stridingType")]
    StridingType(IndexStridingType),
    /// # Information on fields in the original C++ class
    /// -   name:`"flipAlternateTriangles"`
    /// -   type: `hkInt8`
    /// - offset: 47
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "flipAlternateTriangles")]
    FlipAlternateTriangles(i8),
    /// # Information on fields in the original C++ class
    /// -   name:`"extrusion"`
    /// -   type: `hkVector4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extrusion")]
    Extrusion(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"transform"`
    /// -   type: `hkQsTransform`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transform")]
    Transform(cgmath::Matrix4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpExtendedMeshShapeTrianglesSubpartHkParam<'de>, "@name",
    ("numTriangleShapes" => NumTriangleShapes(i32)),
    ("vertexBase" => VertexBase(())),
    ("numVertices" => NumVertices(i32)),
    ("indexBase" => IndexBase(())),
    ("vertexStriding" => VertexStriding(u16)),
    ("triangleOffset" => TriangleOffset(i32)),
    ("indexStriding" => IndexStriding(u16)),
    ("stridingType" => StridingType(IndexStridingType)),
    ("flipAlternateTriangles" => FlipAlternateTriangles(i8)),
    ("extrusion" => Extrusion(cgmath::Vector4<f32>)),
    ("transform" => Transform(cgmath::Matrix4<f32>)),
}
