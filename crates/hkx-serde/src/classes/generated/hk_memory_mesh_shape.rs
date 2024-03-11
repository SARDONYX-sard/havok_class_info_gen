//! A Rust structure that implements a serializer/deserializer corresponding to `hkMemoryMeshShape`, a class defined in C++
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
/// -    size: 48
/// -  vtable: true
/// -  parent: hkMeshShape/`9117d62e`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMemoryMeshShape<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMemoryMeshShape"`: The original C++ class name.
    #[serde(default = "HkMemoryMeshShape::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xb743a578`: Unique value of this class.
    #[serde(default = "HkMemoryMeshShape::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMemoryMeshShapeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMemoryMeshShapeHkParam<'a>>
}

impl HkMemoryMeshShape<'_> {
    /// Return `"hkMemoryMeshShape"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkMemoryMeshShape".into()
    }

    /// Return `"0xb743a578"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xb743a578".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMemoryMeshShapeHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"sections"`
    /// -   type: `hkArray&lt;struct hkMeshSectionCinfo&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sections")]
    Sections(Vec<HkMeshSectionCinfo>),
    /// # Field information in the original C++ class
    /// -   name:`"indices16"`
    /// -   type: `hkArray&lt;hkUint16&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices16")]
    Indices16(Vec<Primitive<u16>>),
    /// # Field information in the original C++ class
    /// -   name:`"indices32"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indices32")]
    Indices32(Vec<Primitive<u32>>),
    /// # Field information in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMemoryMeshShapeHkParam<'de>, "@name",
    ("sections" => Sections(Vec<HkMeshSectionCinfo>)),
    ("indices16" => Indices16(Vec<Primitive<u16>>)),
    ("indices32" => Indices32(Vec<Primitive<u32>>)),
    ("name" => Name(Primitive<Cow<'a, str>>)),
}
