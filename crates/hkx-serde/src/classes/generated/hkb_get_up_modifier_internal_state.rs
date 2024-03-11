//! A Rust structure that implements a serializer/deserializer corresponding to `hkbGetUpModifierInternalState`, a class defined in C++
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
/// -    size: 20
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbGetUpModifierInternalState<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbGetUpModifierInternalState"`: The original C++ class name.
    #[serde(default = "HkbGetUpModifierInternalState::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xd84cad4a`: Unique value of this class.
    #[serde(default = "HkbGetUpModifierInternalState::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbGetUpModifierInternalStateHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbGetUpModifierInternalStateHkParam<'a>>
}

impl HkbGetUpModifierInternalState<'_> {
    /// Return `"hkbGetUpModifierInternalState"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbGetUpModifierInternalState".into()
    }

    /// Return `"0xd84cad4a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xd84cad4a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbGetUpModifierInternalStateHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"timeSinceBegin"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timeSinceBegin")]
    TimeSinceBegin(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"timeStep"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "timeStep")]
    TimeStep(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"initNextModify"`
    /// -   type: `hkBool`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initNextModify")]
    InitNextModify(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbGetUpModifierInternalStateHkParam<'de>, "@name",
    ("timeSinceBegin" => TimeSinceBegin(Primitive<f32>)),
    ("timeStep" => TimeStep(Primitive<f32>)),
    ("initNextModify" => InitNextModify(Primitive<bool>)),
}
