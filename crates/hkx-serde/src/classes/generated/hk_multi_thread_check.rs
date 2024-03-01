//! A Rust structure that implements a serializer/deserializer corresponding to `hkMultiThreadCheck`, a class defined in C++
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
/// -    size: 12
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkMultiThreadCheck<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkMultiThreadCheck"`: Name of this class.
    #[serde(default = "HkMultiThreadCheck::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x11e4408b`: Unique value of this class.
    #[serde(default = "HkMultiThreadCheck::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkMultiThreadCheckHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkMultiThreadCheckHkParam<'a>>
}

impl HkMultiThreadCheck<'_> {
    /// Return `"hkMultiThreadCheck"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkMultiThreadCheck".into()
    }

    /// Return `"0x11e4408b"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x11e4408b".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkMultiThreadCheckHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"threadId"`
    /// -   type: `hkUint32`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "threadId", skip_serializing)]
    ThreadId(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"stackTraceId"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "stackTraceId", skip_serializing)]
    StackTraceId(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"markCount"`
    /// -   type: `hkUint16`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "markCount", skip_serializing)]
    MarkCount(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"markBitStack"`
    /// -   type: `hkUint16`
    /// - offset: 10
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "markBitStack", skip_serializing)]
    MarkBitStack(u16),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkMultiThreadCheckHkParam<'de>, "@name",
    ("threadId" => ThreadId(u32)),
    ("stackTraceId" => StackTraceId(i32)),
    ("markCount" => MarkCount(u16)),
    ("markBitStack" => MarkBitStack(u16)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AccessType {
    #[serde(rename = "HK_ACCESS_IGNORE")]
    HkAccessIgnore = 0,
    #[serde(rename = "HK_ACCESS_RO")]
    HkAccessRo = 1,
    #[serde(rename = "HK_ACCESS_RW")]
    HkAccessRw = 2,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ReadMode {
    #[serde(rename = "THIS_OBJECT_ONLY")]
    ThisObjectOnly = 0,
    #[serde(rename = "RECURSIVE")]
    Recursive = 1,
}
