//! A Rust structure that implements a serializer/deserializer corresponding to `hkxScene`, a class defined in C++
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
/// -    size: 176
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxScene<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxScene"`: The original C++ class name.
    #[serde(default = "HkxScene::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x5f673ddd`: Unique value of this class.
    #[serde(default = "HkxScene::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxSceneHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxSceneHkParam<'a>>
}

impl HkxScene<'_> {
    /// Return `"hkxScene"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkxScene".into()
    }

    /// Return `"0x5f673ddd"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x5f673ddd".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxSceneHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"modeller"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "modeller")]
    Modeller(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"asset"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "asset")]
    Asset(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"sceneLength"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "sceneLength")]
    SceneLength(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"rootNode"`
    /// -   type: `struct hkxNode*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rootNode")]
    RootNode(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"selectionSets"`
    /// -   type: `hkArray&lt;hkxNodeSelectionSet*&gt;`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selectionSets")]
    SelectionSets(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"cameras"`
    /// -   type: `hkArray&lt;hkxCamera*&gt;`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "cameras")]
    Cameras(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"lights"`
    /// -   type: `hkArray&lt;hkxLight*&gt;`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lights")]
    Lights(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"meshes"`
    /// -   type: `hkArray&lt;hkxMesh*&gt;`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "meshes")]
    Meshes(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"materials"`
    /// -   type: `hkArray&lt;hkxMaterial*&gt;`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "materials")]
    Materials(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"inplaceTextures"`
    /// -   type: `hkArray&lt;hkxTextureInplace*&gt;`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "inplaceTextures")]
    InplaceTextures(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"externalTextures"`
    /// -   type: `hkArray&lt;hkxTextureFile*&gt;`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "externalTextures")]
    ExternalTextures(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"skinBindings"`
    /// -   type: `hkArray&lt;hkxSkinBinding*&gt;`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skinBindings")]
    SkinBindings(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"appliedTransform"`
    /// -   type: `hkMatrix3`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "appliedTransform")]
    AppliedTransform(Matrix3<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxSceneHkParam<'de>, "@name",
    ("modeller" => Modeller(Primitive<Cow<'a, str>>)),
    ("asset" => Asset(Primitive<Cow<'a, str>>)),
    ("sceneLength" => SceneLength(Primitive<f32>)),
    ("rootNode" => RootNode(Cow<'a, str>)),
    ("selectionSets" => SelectionSets(Vec<Cow<'a, str>>)),
    ("cameras" => Cameras(Vec<Cow<'a, str>>)),
    ("lights" => Lights(Vec<Cow<'a, str>>)),
    ("meshes" => Meshes(Vec<Cow<'a, str>>)),
    ("materials" => Materials(Vec<Cow<'a, str>>)),
    ("inplaceTextures" => InplaceTextures(Vec<Cow<'a, str>>)),
    ("externalTextures" => ExternalTextures(Vec<Cow<'a, str>>)),
    ("skinBindings" => SkinBindings(Vec<Cow<'a, str>>)),
    ("appliedTransform" => AppliedTransform(Matrix3<f32>)),
}
