//! A Rust structure that implements a serializer/deserializer corresponding to `hkbProjectStringData`, a class defined in C++
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
/// -    size: 76
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbProjectStringData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbProjectStringData"`: The original C++ class name.
    #[serde(default = "HkbProjectStringData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x76ad60a`: Unique value of this class.
    #[serde(default = "HkbProjectStringData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbProjectStringDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbProjectStringDataHkParam<'a>>
}

impl HkbProjectStringData<'_> {
    /// Return `"hkbProjectStringData"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbProjectStringData".into()
    }

    /// Return `"0x76ad60a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x76ad60a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbProjectStringDataHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"animationFilenames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationFilenames")]
    AnimationFilenames(Vec<Primitive<Cow<'a, str>>>),
    /// # Field information in the original C++ class
    /// -   name:`"behaviorFilenames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "behaviorFilenames")]
    BehaviorFilenames(Vec<Primitive<Cow<'a, str>>>),
    /// # Field information in the original C++ class
    /// -   name:`"characterFilenames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterFilenames")]
    CharacterFilenames(Vec<Primitive<Cow<'a, str>>>),
    /// # Field information in the original C++ class
    /// -   name:`"eventNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventNames")]
    EventNames(Vec<Primitive<Cow<'a, str>>>),
    /// # Field information in the original C++ class
    /// -   name:`"animationPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "animationPath")]
    AnimationPath(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"behaviorPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "behaviorPath")]
    BehaviorPath(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"characterPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterPath")]
    CharacterPath(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"fullPathToSource"`
    /// -   type: `hkStringPtr`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fullPathToSource")]
    FullPathToSource(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"rootPath"`
    /// -   type: `hkStringPtr`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "rootPath", skip_serializing)]
    RootPath(Primitive<Cow<'a, str>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbProjectStringDataHkParam<'de>, "@name",
    ("animationFilenames" => AnimationFilenames(Vec<Primitive<Cow<'a, str>>>)),
    ("behaviorFilenames" => BehaviorFilenames(Vec<Primitive<Cow<'a, str>>>)),
    ("characterFilenames" => CharacterFilenames(Vec<Primitive<Cow<'a, str>>>)),
    ("eventNames" => EventNames(Vec<Primitive<Cow<'a, str>>>)),
    ("animationPath" => AnimationPath(Primitive<Cow<'a, str>>)),
    ("behaviorPath" => BehaviorPath(Primitive<Cow<'a, str>>)),
    ("characterPath" => CharacterPath(Primitive<Cow<'a, str>>)),
    ("fullPathToSource" => FullPathToSource(Primitive<Cow<'a, str>>)),
    ("rootPath" => RootPath(Primitive<Cow<'a, str>>)),
}
