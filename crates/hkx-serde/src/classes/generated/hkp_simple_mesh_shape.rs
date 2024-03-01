//! A Rust structure that implements a serializer/deserializer corresponding to `hkpSimpleMeshShape`, a class defined in C++
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
/// -    size: 68
/// -  vtable: true
/// -  parent: hkpShapeCollection/`e8c3991d`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpSimpleMeshShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpSimpleMeshShape"`: Name of this class.
    #[serde(default = "HkpSimpleMeshShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x16b3c811`: Unique value of this class.
    #[serde(default = "HkpSimpleMeshShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpSimpleMeshShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpSimpleMeshShapeHkParam<'a>>
}

impl HkpSimpleMeshShape<'_> {
    /// Return `"hkpSimpleMeshShape"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpSimpleMeshShape".into()
    }

    /// Return `"0x16b3c811"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x16b3c811".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSimpleMeshShapeHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"vertices"`
    /// -   type: `hkArray&lt;hkVector4&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertices")]
    Vertices(Vec<cgmath::Vector4<f32>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"triangles"`
    /// -   type: `hkArray&lt;struct hkpSimpleMeshShapeTriangle&gt;`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "triangles")]
    Triangles(Vec<HkpSimpleMeshShapeTriangle>),
    /// # Information on fields in the original C++ class
    /// -   name:`"materialIndices"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materialIndices")]
    MaterialIndices(Vec<u8>),
    /// # Information on fields in the original C++ class
    /// -   name:`"radius"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "radius")]
    Radius(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"weldingType"`
    /// -   type: `enum WeldingType`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "weldingType")]
    WeldingType(WeldingType),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpSimpleMeshShapeHkParam<'de>, "@name",
    ("vertices" => Vertices(Vec<cgmath::Vector4<f32>>)),
    ("triangles" => Triangles(Vec<HkpSimpleMeshShapeTriangle>)),
    ("materialIndices" => MaterialIndices(Vec<u8>)),
    ("radius" => Radius(f64)),
    ("weldingType" => WeldingType(WeldingType)),
}
