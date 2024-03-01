//! A Rust structure that implements a serializer/deserializer corresponding to `hkpPhantom`, a class defined in C++
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
/// -    size: 164
/// -  vtable: true
/// -  parent: hkpWorldObject/`49fb6f2e`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpPhantom<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpPhantom"`: Name of this class.
    #[serde(default = "HkpPhantom::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x9b7e6f86`: Unique value of this class.
    #[serde(default = "HkpPhantom::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpPhantomHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpPhantomHkParam<'a>>
}

impl HkpPhantom<'_> {
    /// Return `"hkpPhantom"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpPhantom".into()
    }

    /// Return `"0x9b7e6f86"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x9b7e6f86".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpPhantomHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"overlapListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 140
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "overlapListeners", skip_serializing)]
    OverlapListeners(Vec<()>),
    /// # Information on fields in the original C++ class
    /// -   name:`"phantomListeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 152
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "phantomListeners", skip_serializing)]
    PhantomListeners(Vec<()>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpPhantomHkParam<'de>, "@name",
    ("overlapListeners" => OverlapListeners(Vec<()>)),
    ("phantomListeners" => PhantomListeners(Vec<()>)),
}
