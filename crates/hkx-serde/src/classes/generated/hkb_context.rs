//! A Rust structure that implements a serializer/deserializer corresponding to `hkbContext`, a class defined in C++
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
/// -    size: 40
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbContext<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbContext"`: The original C++ class name.
    #[serde(default = "HkbContext::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xe0c4d4a7`: Unique value of this class.
    #[serde(default = "HkbContext::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbContextHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbContextHkParam<'a>>
}

impl HkbContext<'_> {
    /// Return `"hkbContext"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbContext".into()
    }

    /// Return `"0xe0c4d4a7"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xe0c4d4a7".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbContextHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"character"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "character", skip_serializing)]
    Character(()),
    /// # Field information in the original C++ class
    /// -   name:`"behavior"`
    /// -   type: `void*`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "behavior", skip_serializing)]
    Behavior(()),
    /// # Field information in the original C++ class
    /// -   name:`"nodeToIndexMap"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "nodeToIndexMap", skip_serializing)]
    NodeToIndexMap(()),
    /// # Field information in the original C++ class
    /// -   name:`"eventQueue"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "eventQueue", skip_serializing)]
    EventQueue(()),
    /// # Field information in the original C++ class
    /// -   name:`"sharedEventQueue"`
    /// -   type: `void*`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "sharedEventQueue", skip_serializing)]
    SharedEventQueue(()),
    /// # Field information in the original C++ class
    /// -   name:`"generatorOutputListener"`
    /// -   type: `struct hkbGeneratorOutputListener*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "generatorOutputListener")]
    GeneratorOutputListener(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"eventTriggeredTransition"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "eventTriggeredTransition", skip_serializing)]
    EventTriggeredTransition(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "world", skip_serializing)]
    World(()),
    /// # Field information in the original C++ class
    /// -   name:`"attachmentManager"`
    /// -   type: `void*`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "attachmentManager", skip_serializing)]
    AttachmentManager(()),
    /// # Field information in the original C++ class
    /// -   name:`"animationCache"`
    /// -   type: `void*`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "animationCache", skip_serializing)]
    AnimationCache(()),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbContextHkParam<'de>, "@name",
    ("character" => Character(())),
    ("behavior" => Behavior(())),
    ("nodeToIndexMap" => NodeToIndexMap(())),
    ("eventQueue" => EventQueue(())),
    ("sharedEventQueue" => SharedEventQueue(())),
    ("generatorOutputListener" => GeneratorOutputListener(Cow<'a, str>)),
    ("eventTriggeredTransition" => EventTriggeredTransition(Primitive<bool>)),
    ("world" => World(())),
    ("attachmentManager" => AttachmentManager(())),
    ("animationCache" => AnimationCache(())),
}
