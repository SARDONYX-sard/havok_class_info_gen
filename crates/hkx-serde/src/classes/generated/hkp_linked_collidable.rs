//! A Rust structure that implements a serializer/deserializer corresponding to `hkpLinkedCollidable`, a class defined in C++
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
/// -    size: 92
/// -  vtable: false
/// -  parent: hkpCollidable/`9a0e42a5`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpLinkedCollidable<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpLinkedCollidable"`: The original C++ class name.
    #[serde(default = "HkpLinkedCollidable::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xe1a81497`: Unique value of this class.
    #[serde(default = "HkpLinkedCollidable::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpLinkedCollidableHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpLinkedCollidableHkParam<'a>>
}

impl HkpLinkedCollidable<'_> {
    /// Return `"hkpLinkedCollidable"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpLinkedCollidable".into()
    }

    /// Return `"0xe1a81497"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xe1a81497".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpLinkedCollidableHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"collisionEntries"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "collisionEntries", skip_serializing)]
    CollisionEntries(Vec<()>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpLinkedCollidableHkParam<'de>, "@name",
    ("collisionEntries" => CollisionEntries(Vec<()>)),
}
