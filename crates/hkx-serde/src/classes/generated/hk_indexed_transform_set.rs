//! A Rust structure that implements a serializer/deserializer corresponding to `hkIndexedTransformSet`, a class defined in C++
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
/// -    size: 72
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkIndexedTransformSet<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkIndexedTransformSet"`: Name of this class.
    #[serde(default = "HkIndexedTransformSet::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x87fe6b5c`: Unique value of this class.
    #[serde(default = "HkIndexedTransformSet::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkIndexedTransformSetHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkIndexedTransformSetHkParam<'a>>
}

impl HkIndexedTransformSet<'_> {
    /// Return `"hkIndexedTransformSet"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkIndexedTransformSet".into()
    }

    /// Return `"0x87fe6b5c"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x87fe6b5c".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkIndexedTransformSetHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"matrices"`
    /// -   type: `hkArray&lt;hkMatrix4&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "matrices")]
    Matrices(Vec<cgmath::Matrix4<f32>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"inverseMatrices"`
    /// -   type: `hkArray&lt;hkMatrix4&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "inverseMatrices")]
    InverseMatrices(Vec<cgmath::Matrix4<f32>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"matricesOrder"`
    /// -   type: `hkArray&lt;hkInt16&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "matricesOrder")]
    MatricesOrder(Vec<i16>),
    /// # Information on fields in the original C++ class
    /// -   name:`"matricesNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "matricesNames")]
    MatricesNames(Vec<String>),
    /// # Information on fields in the original C++ class
    /// -   name:`"indexMappings"`
    /// -   type: `hkArray&lt;struct hkMeshBoneIndexMapping&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexMappings")]
    IndexMappings(Vec<HkMeshBoneIndexMapping>),
    /// # Information on fields in the original C++ class
    /// -   name:`"allMatricesAreAffine"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allMatricesAreAffine")]
    AllMatricesAreAffine(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkIndexedTransformSetHkParam<'de>, "@name",
    ("matrices" => Matrices(Vec<cgmath::Matrix4<f32>>)),
    ("inverseMatrices" => InverseMatrices(Vec<cgmath::Matrix4<f32>>)),
    ("matricesOrder" => MatricesOrder(Vec<i16>)),
    ("matricesNames" => MatricesNames(Vec<String>)),
    ("indexMappings" => IndexMappings(Vec<HkMeshBoneIndexMapping>)),
    ("allMatricesAreAffine" => AllMatricesAreAffine(bool)),
}
