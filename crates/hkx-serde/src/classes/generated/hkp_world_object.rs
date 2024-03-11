//! A Rust structure that implements a serializer/deserializer corresponding to `hkpWorldObject`, a class defined in C++
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
/// -    size: 140
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpWorldObject<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpWorldObject"`: The original C++ class name.
    #[serde(default = "HkpWorldObject::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x49fb6f2e`: Unique value of this class.
    #[serde(default = "HkpWorldObject::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpWorldObjectHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpWorldObjectHkParam<'a>>
}

impl HkpWorldObject<'_> {
    /// Return `"hkpWorldObject"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpWorldObject".into()
    }

    /// Return `"0x49fb6f2e"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x49fb6f2e".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpWorldObjectHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "world", skip_serializing)]
    World(()),
    /// # Field information in the original C++ class
    /// -   name:`"userData"`
    /// -   type: `hkUlong`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userData")]
    UserData(Primitive<u64>),
    /// # Field information in the original C++ class
    /// -   name:`"collidable"`
    /// -   type: `struct hkpLinkedCollidable`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collidable")]
    Collidable(HkpLinkedCollidable),
    /// # Field information in the original C++ class
    /// -   name:`"multiThreadCheck"`
    /// -   type: `struct hkMultiThreadCheck`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "multiThreadCheck")]
    MultiThreadCheck(HkMultiThreadCheck),
    /// # Field information in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"properties"`
    /// -   type: `hkArray&lt;struct hkpProperty&gt;`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "properties")]
    Properties(Vec<HkpProperty>),
    /// # Field information in the original C++ class
    /// -   name:`"treeData"`
    /// -   type: `void*`
    /// - offset: 136
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "treeData", skip_serializing)]
    TreeData(()),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpWorldObjectHkParam<'de>, "@name",
    ("world" => World(())),
    ("userData" => UserData(Primitive<u64>)),
    ("collidable" => Collidable(HkpLinkedCollidable)),
    ("multiThreadCheck" => MultiThreadCheck(HkMultiThreadCheck)),
    ("name" => Name(Primitive<Cow<'a, str>>)),
    ("properties" => Properties(Vec<HkpProperty>)),
    ("treeData" => TreeData(())),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MtChecks {
    #[serde(rename = "MULTI_THREADING_CHECKS_ENABLE")]
    MultiThreadingChecksEnable = 0,
    #[serde(rename = "MULTI_THREADING_CHECKS_IGNORE")]
    MultiThreadingChecksIgnore = 1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BroadPhaseType {
    #[serde(rename = "BROAD_PHASE_INVALID")]
    BroadPhaseInvalid = 0,
    #[serde(rename = "BROAD_PHASE_ENTITY")]
    BroadPhaseEntity = 1,
    #[serde(rename = "BROAD_PHASE_PHANTOM")]
    BroadPhasePhantom = 2,
    #[serde(rename = "BROAD_PHASE_BORDER")]
    BroadPhaseBorder = 3,
    #[serde(rename = "BROAD_PHASE_MAX_ID")]
    BroadPhaseMaxId = 4,
}
