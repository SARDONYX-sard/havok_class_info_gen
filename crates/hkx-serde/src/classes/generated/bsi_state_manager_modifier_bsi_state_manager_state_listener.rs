//! A Rust structure that implements a serializer/deserializer corresponding to `BSIStateManagerModifierBSIStateManagerStateListener`, a class defined in C++
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
/// -  vtable: true
/// -  parent: hkbStateListener/`da8c7d7d`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsiStateManagerModifierBsiStateManagerStateListener<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSIStateManagerModifierBSIStateManagerStateListener"`: Name of this class.
    #[serde(default = "BsiStateManagerModifierBsiStateManagerStateListener::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x99463586`: Unique value of this class.
    #[serde(default = "BsiStateManagerModifierBsiStateManagerStateListener::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsiStateManagerModifierBsiStateManagerStateListenerHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsiStateManagerModifierBsiStateManagerStateListenerHkParam<'a>>
}

impl BsiStateManagerModifierBsiStateManagerStateListener<'_> {
    /// Return `"BSIStateManagerModifierBSIStateManagerStateListener"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BsiStateManagerModifierBsiStateManagerStateListener".into()
    }

    /// Return `"0x99463586"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x99463586".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsiStateManagerModifierBsiStateManagerStateListenerHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"pStateManager"`
    /// -   type: `void*`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "pStateManager", skip_serializing)]
    PStateManager(()),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsiStateManagerModifierBsiStateManagerStateListenerHkParam<'de>, "@name",
    ("pStateManager" => PStateManager(())),
}
