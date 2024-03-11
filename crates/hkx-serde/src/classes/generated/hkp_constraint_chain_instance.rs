//! A Rust structure that implements a serializer/deserializer corresponding to `hkpConstraintChainInstance`, a class defined in C++
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
/// -  parent: hkpConstraintInstance/`34eba5f`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpConstraintChainInstance<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpConstraintChainInstance"`: The original C++ class name.
    #[serde(default = "HkpConstraintChainInstance::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x7a490753`: Unique value of this class.
    #[serde(default = "HkpConstraintChainInstance::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpConstraintChainInstanceHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpConstraintChainInstanceHkParam<'a>>
}

impl HkpConstraintChainInstance<'_> {
    /// Return `"hkpConstraintChainInstance"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpConstraintChainInstance".into()
    }

    /// Return `"0x7a490753"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x7a490753".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpConstraintChainInstanceHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"chainedEntities"`
    /// -   type: `hkArray&lt;hkpEntity*&gt;`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "chainedEntities")]
    ChainedEntities(Vec<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"action"`
    /// -   type: `struct hkpConstraintChainInstanceAction*`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "action")]
    Action(Cow<'a, str>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpConstraintChainInstanceHkParam<'de>, "@name",
    ("chainedEntities" => ChainedEntities(Vec<Cow<'a, str>>)),
    ("action" => Action(Cow<'a, str>)),
}
