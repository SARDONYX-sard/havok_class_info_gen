//! A Rust structure that implements a serializer/deserializer corresponding to `hkMemoryMeshBody`, a class defined in C++
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
/// -    size: 112
/// -  vtable: true
/// -  parent: hkMeshBody/`d0be5d7d`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMemoryMeshBody<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMemoryMeshBody"`: The original C++ class name.
    #[serde(default = "HkMemoryMeshBody::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x94a620a8`: Unique value of this class.
    #[serde(default = "HkMemoryMeshBody::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMemoryMeshBodyHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMemoryMeshBodyHkParam<'a>>
}

impl HkMemoryMeshBody<'_> {
    /// Return `"hkMemoryMeshBody"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkMemoryMeshBody".into()
    }

    /// Return `"0x94a620a8"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x94a620a8".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMemoryMeshBodyHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"transform"`
    /// -   type: `hkMatrix4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transform")]
    Transform(Matrix4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"transformSet"`
    /// -   type: `struct hkIndexedTransformSet*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformSet")]
    TransformSet(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"shape"`
    /// -   type: `struct hkMeshShape*`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shape")]
    Shape(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"vertexBuffers"`
    /// -   type: `hkArray&lt;hkMeshVertexBuffer*&gt;`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vertexBuffers")]
    VertexBuffers(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryMeshBodyHkParam<'de>, "@name",
    ("transform" => Transform(Matrix4<f32>)),
    ("transformSet" => TransformSet(Cow<'a, str>)),
    ("shape" => Shape(Cow<'a, str>)),
    ("vertexBuffers" => VertexBuffers(Vec<Cow<'a, str>>)),
    ("name" => Name(Primitive<Cow<'a, str>>)),
}
