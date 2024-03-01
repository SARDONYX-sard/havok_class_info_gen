//! A Rust structure that implements a serializer/deserializer corresponding to `hkpConvexVerticesConnectivity`, a class defined in C++
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
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpConvexVerticesConnectivity<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpConvexVerticesConnectivity"`: Name of this class.
    #[serde(default = "HkpConvexVerticesConnectivity::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x63d38e9c`: Unique value of this class.
    #[serde(default = "HkpConvexVerticesConnectivity::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpConvexVerticesConnectivityHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpConvexVerticesConnectivityHkParam<'a>>
}

impl HkpConvexVerticesConnectivity<'_> {
    /// Return `"hkpConvexVerticesConnectivity"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpConvexVerticesConnectivity".into()
    }

    /// Return `"0x63d38e9c"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x63d38e9c".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConvexVerticesConnectivityHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"vertexIndices"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexIndices")]
    VertexIndices(Vec<u16>),
    /// # Information on fields in the original C++ class
    /// -   name:`"numVerticesPerFace"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numVerticesPerFace")]
    NumVerticesPerFace(Vec<u8>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpConvexVerticesConnectivityHkParam<'de>, "@name",
    ("vertexIndices" => VertexIndices(Vec<u16>)),
    ("numVerticesPerFace" => NumVerticesPerFace(Vec<u8>)),
}
