//! A Rust structure that implements a serializer/deserializer corresponding to `hkpDisableEntityCollisionFilter`, a class defined in C++
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
/// -    size: 64
/// -  vtable: true
/// -  parent: hkpCollisionFilter/`60960336`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpDisableEntityCollisionFilter<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpDisableEntityCollisionFilter"`: The original C++ class name.
    #[serde(default = "HkpDisableEntityCollisionFilter::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xfac3351c`: Unique value of this class.
    #[serde(default = "HkpDisableEntityCollisionFilter::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpDisableEntityCollisionFilterHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpDisableEntityCollisionFilterHkParam<'a>>
}

impl HkpDisableEntityCollisionFilter<'_> {
    /// Return `"hkpDisableEntityCollisionFilter"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpDisableEntityCollisionFilter".into()
    }

    /// Return `"0xfac3351c"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xfac3351c".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpDisableEntityCollisionFilterHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"disabledEntities"`
    /// -   type: `hkArray&lt;hkpEntity*&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "disabledEntities")]
    DisabledEntities(Vec<Cow<'a, str>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpDisableEntityCollisionFilterHkParam<'de>, "@name",
    ("disabledEntities" => DisabledEntities(Vec<Cow<'a, str>>)),
}
