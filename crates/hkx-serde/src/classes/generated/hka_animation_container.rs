//! A Rust structure that implements a serializer/deserializer corresponding to `hkaAnimationContainer`, a class defined in C++
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
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaAnimationContainer<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaAnimationContainer"`: Name of this class.
    #[serde(default = "HkaAnimationContainer::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x8dc20333`: Unique value of this class.
    #[serde(default = "HkaAnimationContainer::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaAnimationContainerHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaAnimationContainerHkParam<'a>>
}

impl HkaAnimationContainer<'_> {
    /// Return `"hkaAnimationContainer"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkaAnimationContainer".into()
    }

    /// Return `"0x8dc20333"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x8dc20333".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaAnimationContainerHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"skeletons"`
    /// -   type: `hkArray&lt;hkaSkeleton*&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skeletons")]
    Skeletons(Vec<Box<HkaSkeleton>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"animations"`
    /// -   type: `hkArray&lt;hkaAnimation*&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animations")]
    Animations(Vec<Box<HkaAnimation>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"bindings"`
    /// -   type: `hkArray&lt;hkaAnimationBinding*&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bindings")]
    Bindings(Vec<Box<HkaAnimationBinding>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"attachments"`
    /// -   type: `hkArray&lt;hkaBoneAttachment*&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attachments")]
    Attachments(Vec<Box<HkaBoneAttachment>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"skins"`
    /// -   type: `hkArray&lt;hkaMeshBinding*&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "skins")]
    Skins(Vec<Box<HkaMeshBinding>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaAnimationContainerHkParam<'de>, "@name",
    ("skeletons" => Skeletons(Vec<Box<HkaSkeleton>>)),
    ("animations" => Animations(Vec<Box<HkaAnimation>>)),
    ("bindings" => Bindings(Vec<Box<HkaAnimationBinding>>)),
    ("attachments" => Attachments(Vec<Box<HkaBoneAttachment>>)),
    ("skins" => Skins(Vec<Box<HkaMeshBinding>>)),
}
