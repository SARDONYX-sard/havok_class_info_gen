//! A Rust structure that implements a serializer/deserializer corresponding to `hkaMeshBinding`, a class defined in C++
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
/// -    size: 44
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaMeshBinding<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaMeshBinding"`: The original C++ class name.
    #[serde(default = "HkaMeshBinding::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x81d9950b`: Unique value of this class.
    #[serde(default = "HkaMeshBinding::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaMeshBindingHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaMeshBindingHkParam<'a>>
}

impl HkaMeshBinding<'_> {
    /// Return `"hkaMeshBinding"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkaMeshBinding".into()
    }

    /// Return `"0x81d9950b"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x81d9950b".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaMeshBindingHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"mesh"`
    /// -   type: `struct hkxMesh*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mesh")]
    Mesh(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"originalSkeletonName"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "originalSkeletonName")]
    OriginalSkeletonName(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"skeleton"`
    /// -   type: `struct hkaSkeleton*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skeleton")]
    Skeleton(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"mappings"`
    /// -   type: `hkArray&lt;struct hkaMeshBindingMapping&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mappings")]
    Mappings(Vec<HkaMeshBindingMapping>),
    /// # Field information in the original C++ class
    /// -   name:`"boneFromSkinMeshTransforms"`
    /// -   type: `hkArray&lt;hkTransform&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "boneFromSkinMeshTransforms")]
    BoneFromSkinMeshTransforms(Vec<Transform<f32>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaMeshBindingHkParam<'de>, "@name",
    ("mesh" => Mesh(Cow<'a, str>)),
    ("originalSkeletonName" => OriginalSkeletonName(Primitive<Cow<'a, str>>)),
    ("skeleton" => Skeleton(Cow<'a, str>)),
    ("mappings" => Mappings(Vec<HkaMeshBindingMapping>)),
    ("boneFromSkinMeshTransforms" => BoneFromSkinMeshTransforms(Vec<Transform<f32>>)),
}
