//! A Rust structure that implements a serializer/deserializer corresponding to `hkpPhysicsSystemWithContacts`, a class defined in C++
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
/// -    size: 80
/// -  vtable: true
/// -  parent: hkpPhysicsSystem/`ff724c17`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpPhysicsSystemWithContacts<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpPhysicsSystemWithContacts"`: Name of this class.
    #[serde(default = "HkpPhysicsSystemWithContacts::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xd0fd4bbe`: Unique value of this class.
    #[serde(default = "HkpPhysicsSystemWithContacts::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpPhysicsSystemWithContactsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpPhysicsSystemWithContactsHkParam<'a>>
}

impl HkpPhysicsSystemWithContacts<'_> {
    /// Return `"hkpPhysicsSystemWithContacts"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpPhysicsSystemWithContacts".into()
    }

    /// Return `"0xd0fd4bbe"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xd0fd4bbe".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPhysicsSystemWithContactsHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"contacts"`
    /// -   type: `hkArray&lt;hkpSerializedAgentNnEntry*&gt;`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contacts")]
    Contacts(Vec<Box<HkpSerializedAgentNnEntry>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpPhysicsSystemWithContactsHkParam<'de>, "@name",
    ("contacts" => Contacts(Vec<Box<HkpSerializedAgentNnEntry>>)),
}
