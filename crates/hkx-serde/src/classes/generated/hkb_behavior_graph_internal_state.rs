//! A Rust structure that implements a serializer/deserializer corresponding to `hkbBehaviorGraphInternalState`, a class defined in C++
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
/// -    size: 24
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbBehaviorGraphInternalState<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbBehaviorGraphInternalState"`: Name of this class.
    #[serde(default = "HkbBehaviorGraphInternalState::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x8699b6eb`: Unique value of this class.
    #[serde(default = "HkbBehaviorGraphInternalState::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbBehaviorGraphInternalStateHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbBehaviorGraphInternalStateHkParam<'a>>
}

impl HkbBehaviorGraphInternalState<'_> {
    /// Return `"hkbBehaviorGraphInternalState"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbBehaviorGraphInternalState".into()
    }

    /// Return `"0x8699b6eb"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x8699b6eb".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBehaviorGraphInternalStateHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"nodeInternalStateInfos"`
    /// -   type: `hkArray&lt;hkbNodeInternalStateInfo*&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "nodeInternalStateInfos")]
    NodeInternalStateInfos(Vec<Box<HkbNodeInternalStateInfo>>),
    /// # Information on fields in the original C++ class
    /// -   name:`"variableValueSet"`
    /// -   type: `struct hkbVariableValueSet*`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableValueSet")]
    VariableValueSet(Box<HkbVariableValueSet>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorGraphInternalStateHkParam<'de>, "@name",
    ("nodeInternalStateInfos" => NodeInternalStateInfos(Vec<Box<HkbNodeInternalStateInfo>>)),
    ("variableValueSet" => VariableValueSet(Box<HkbVariableValueSet>)),
}
