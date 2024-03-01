//! A Rust structure that implements a serializer/deserializer corresponding to `BSIStateManagerModifierBSiStateData`, a class defined in C++
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
pub struct BsiStateManagerModifierBSiStateData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSIStateManagerModifierBSiStateData"`: Name of this class.
    #[serde(default = "BsiStateManagerModifierBSiStateData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x6b8a15fc`: Unique value of this class.
    #[serde(default = "BsiStateManagerModifierBSiStateData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsiStateManagerModifierBSiStateDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsiStateManagerModifierBSiStateDataHkParam<'a>>
}

impl BsiStateManagerModifierBSiStateData<'_> {
    /// Return `"BSIStateManagerModifierBSiStateData"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BsiStateManagerModifierBSiStateData".into()
    }

    /// Return `"0x6b8a15fc"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x6b8a15fc".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsiStateManagerModifierBSiStateDataHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"pStateMachine"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pStateMachine")]
    PStateMachine(Box<HkbGenerator>),
    /// # Information on fields in the original C++ class
    /// -   name:`"StateID"`
    /// -   type: `hkInt32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "StateID")]
    StateId(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"iStateToSetAs"`
    /// -   type: `hkInt32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "iStateToSetAs")]
    IStateToSetAs(i32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsiStateManagerModifierBSiStateDataHkParam<'de>, "@name",
    ("pStateMachine" => PStateMachine(Box<HkbGenerator>)),
    ("StateID" => StateId(i32)),
    ("iStateToSetAs" => IStateToSetAs(i32)),
}
