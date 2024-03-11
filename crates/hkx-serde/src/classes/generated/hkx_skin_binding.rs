//! A Rust structure that implements a serializer/deserializer corresponding to `hkxSkinBinding`, a class defined in C++
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
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxSkinBinding<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxSkinBinding"`: The original C++ class name.
    #[serde(default = "HkxSkinBinding::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x5a93f338`: Unique value of this class.
    #[serde(default = "HkxSkinBinding::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxSkinBindingHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxSkinBindingHkParam<'a>>
}

impl HkxSkinBinding<'_> {
    /// Return `"hkxSkinBinding"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkxSkinBinding".into()
    }

    /// Return `"0x5a93f338"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x5a93f338".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxSkinBindingHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"mesh"`
    /// -   type: `struct hkxMesh*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mesh")]
    Mesh(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"nodeNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nodeNames")]
    NodeNames(Vec<Primitive<Cow<'a, str>>>),
    /// # Field information in the original C++ class
    /// -   name:`"bindPose"`
    /// -   type: `hkArray&lt;hkMatrix4&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bindPose")]
    BindPose(Vec<Matrix4<f32>>),
    /// # Field information in the original C++ class
    /// -   name:`"initSkinTransform"`
    /// -   type: `hkMatrix4`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initSkinTransform")]
    InitSkinTransform(Matrix4<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxSkinBindingHkParam<'de>, "@name",
    ("mesh" => Mesh(Cow<'a, str>)),
    ("nodeNames" => NodeNames(Vec<Primitive<Cow<'a, str>>>)),
    ("bindPose" => BindPose(Vec<Matrix4<f32>>)),
    ("initSkinTransform" => InitSkinTransform(Matrix4<f32>)),
}
