//! A Rust structure that implements a serializer/deserializer corresponding to `BSIStateManagerModifier`, a class defined in C++
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
/// -    size: 72
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsiStateManagerModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSIStateManagerModifier"`: The original C++ class name.
    #[serde(default = "BsiStateManagerModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x6cb24f2e`: Unique value of this class.
    #[serde(default = "BsiStateManagerModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsiStateManagerModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsiStateManagerModifierHkParam<'a>>
}

impl BsiStateManagerModifier<'_> {
    /// Return `"BSIStateManagerModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BSIStateManagerModifier".into()
    }

    /// Return `"0x6cb24f2e"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x6cb24f2e".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsiStateManagerModifierHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"iStateVar"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "iStateVar")]
    IStateVar(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"stateData"`
    /// -   type: `hkArray&lt;struct BSIStateManagerModifierBSiStateData&gt;`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "stateData")]
    StateData(Vec<BsiStateManagerModifierBSiStateData>),
    /// # Field information in the original C++ class
    /// -   name:`"myStateListener"`
    /// -   type: `struct BSIStateManagerModifierBSIStateManagerStateListener`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "myStateListener", skip_serializing)]
    MyStateListener(BsiStateManagerModifierBsiStateManagerStateListener),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsiStateManagerModifierHkParam<'de>, "@name",
    ("iStateVar" => IStateVar(Primitive<i32>)),
    ("stateData" => StateData(Vec<BsiStateManagerModifierBSiStateData>)),
    ("myStateListener" => MyStateListener(BsiStateManagerModifierBsiStateManagerStateListener)),
}
