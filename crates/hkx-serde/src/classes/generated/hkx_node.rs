//! A Rust structure that implements a serializer/deserializer corresponding to `hkxNode`, a class defined in C++
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
/// -    size: 72
/// -  vtable: true
/// -  parent: hkxAttributeHolder/`7468cc44`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkxNode<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkxNode"`: The original C++ class name.
    #[serde(default = "HkxNode::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x5a218502`: Unique value of this class.
    #[serde(default = "HkxNode::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkxNodeHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkxNodeHkParam<'a>>
}

impl HkxNode<'_> {
    /// Return `"hkxNode"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkxNode".into()
    }

    /// Return `"0x5a218502"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x5a218502".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkxNodeHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"object"`
    /// -   type: `struct hkReferencedObject*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "object")]
    Object(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"keyFrames"`
    /// -   type: `hkArray&lt;hkMatrix4&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keyFrames")]
    KeyFrames(Vec<Matrix4<f32>>),
    /// # Field information in the original C++ class
    /// -   name:`"children"`
    /// -   type: `hkArray&lt;hkxNode*&gt;`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "children")]
    Children(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"annotations"`
    /// -   type: `hkArray&lt;struct hkxNodeAnnotationData&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "annotations")]
    Annotations(Vec<HkxNodeAnnotationData>),
    /// # Field information in the original C++ class
    /// -   name:`"userProperties"`
    /// -   type: `hkStringPtr`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userProperties")]
    UserProperties(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"selected"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "selected")]
    Selected(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkxNodeHkParam<'de>, "@name",
    ("name" => Name(Primitive<Cow<'a, str>>)),
    ("object" => Object(Cow<'a, str>)),
    ("keyFrames" => KeyFrames(Vec<Matrix4<f32>>)),
    ("children" => Children(Vec<Cow<'a, str>>)),
    ("annotations" => Annotations(Vec<HkxNodeAnnotationData>)),
    ("userProperties" => UserProperties(Primitive<Cow<'a, str>>)),
    ("selected" => Selected(Primitive<bool>)),
}
