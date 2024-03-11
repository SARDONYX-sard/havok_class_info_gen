//! A Rust structure that implements a serializer/deserializer corresponding to `hkAlignSceneToNodeOptions`, a class defined in C++
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
/// -    size: 24
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkAlignSceneToNodeOptions<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkAlignSceneToNodeOptions"`: The original C++ class name.
    #[serde(default = "HkAlignSceneToNodeOptions::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x207cb01`: Unique value of this class.
    #[serde(default = "HkAlignSceneToNodeOptions::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkAlignSceneToNodeOptionsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkAlignSceneToNodeOptionsHkParam<'a>>
}

impl HkAlignSceneToNodeOptions<'_> {
    /// Return `"hkAlignSceneToNodeOptions"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkAlignSceneToNodeOptions".into()
    }

    /// Return `"0x207cb01"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x207cb01".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkAlignSceneToNodeOptionsHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"invert"`
    /// -   type: `hkBool`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "invert")]
    Invert(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"transformPositionX"`
    /// -   type: `hkBool`
    /// - offset: 9
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformPositionX")]
    TransformPositionX(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"transformPositionY"`
    /// -   type: `hkBool`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformPositionY")]
    TransformPositionY(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"transformPositionZ"`
    /// -   type: `hkBool`
    /// - offset: 11
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformPositionZ")]
    TransformPositionZ(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"transformRotation"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformRotation")]
    TransformRotation(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"transformScale"`
    /// -   type: `hkBool`
    /// - offset: 13
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformScale")]
    TransformScale(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"transformSkew"`
    /// -   type: `hkBool`
    /// - offset: 14
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformSkew")]
    TransformSkew(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"keyframe"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keyframe")]
    Keyframe(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"nodeName"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nodeName")]
    NodeName(Primitive<Cow<'a, str>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkAlignSceneToNodeOptionsHkParam<'de>, "@name",
    ("invert" => Invert(Primitive<bool>)),
    ("transformPositionX" => TransformPositionX(Primitive<bool>)),
    ("transformPositionY" => TransformPositionY(Primitive<bool>)),
    ("transformPositionZ" => TransformPositionZ(Primitive<bool>)),
    ("transformRotation" => TransformRotation(Primitive<bool>)),
    ("transformScale" => TransformScale(Primitive<bool>)),
    ("transformSkew" => TransformSkew(Primitive<bool>)),
    ("keyframe" => Keyframe(Primitive<i32>)),
    ("nodeName" => NodeName(Primitive<Cow<'a, str>>)),
}
