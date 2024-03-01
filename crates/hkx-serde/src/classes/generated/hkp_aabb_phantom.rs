//! A Rust structure that implements a serializer/deserializer corresponding to `hkpAabbPhantom`, a class defined in C++
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
/// -    size: 224
/// -  vtable: true
/// -  parent: hkpPhantom/`9b7e6f86`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpAabbPhantom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpAabbPhantom"`: Name of this class.
    #[serde(default = "HkpAabbPhantom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x2c5189dd`: Unique value of this class.
    #[serde(default = "HkpAabbPhantom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpAabbPhantomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpAabbPhantomHkParam<'a>>
}

impl HkpAabbPhantom<'_> {
    /// Return `"hkpAabbPhantom"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpAabbPhantom".into()
    }

    /// Return `"0x2c5189dd"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x2c5189dd".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpAabbPhantomHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"aabb"`
    /// -   type: `struct hkAabb`
    /// - offset: 176
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "aabb")]
    Aabb(HkAabb),
    /// # Information on fields in the original C++ class
    /// -   name:`"overlappingCollidables"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 208
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "overlappingCollidables", skip_serializing)]
    OverlappingCollidables(Vec<()>),
    /// # Information on fields in the original C++ class
    /// -   name:`"orderDirty"`
    /// -   type: `hkBool`
    /// - offset: 220
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "orderDirty", skip_serializing)]
    OrderDirty(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpAabbPhantomHkParam<'de>, "@name",
    ("aabb" => Aabb(HkAabb)),
    ("overlappingCollidables" => OverlappingCollidables(Vec<()>)),
    ("orderDirty" => OrderDirty(bool)),
}
