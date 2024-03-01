//! A Rust structure that implements a serializer/deserializer corresponding to `hkpEntitySpuCollisionCallback`, a class defined in C++
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
/// -    size: 8
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpEntitySpuCollisionCallback<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpEntitySpuCollisionCallback"`: Name of this class.
    #[serde(default = "HkpEntitySpuCollisionCallback::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x81147f05`: Unique value of this class.
    #[serde(default = "HkpEntitySpuCollisionCallback::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpEntitySpuCollisionCallbackHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpEntitySpuCollisionCallbackHkParam<'a>>
}

impl HkpEntitySpuCollisionCallback<'_> {
    /// Return `"hkpEntitySpuCollisionCallback"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpEntitySpuCollisionCallback".into()
    }

    /// Return `"0x81147f05"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x81147f05".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpEntitySpuCollisionCallbackHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"util"`
    /// -   type: `void*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "util", skip_serializing)]
    Util(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"capacity"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "capacity", skip_serializing)]
    Capacity(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"eventFilter"`
    /// -   type: `hkUint8`
    /// - offset: 6
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventFilter")]
    EventFilter(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"userFilter"`
    /// -   type: `hkUint8`
    /// - offset: 7
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userFilter")]
    UserFilter(u8),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpEntitySpuCollisionCallbackHkParam<'de>, "@name",
    ("util" => Util(())),
    ("capacity" => Capacity(u16)),
    ("eventFilter" => EventFilter(u8)),
    ("userFilter" => UserFilter(u8)),
}
