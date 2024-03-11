//! A Rust structure that implements a serializer/deserializer corresponding to `hkbEventDrivenModifier`, a class defined in C++
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
/// -    size: 60
/// -  vtable: true
/// -  parent: hkbModifierWrapper/`3697e044`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbEventDrivenModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbEventDrivenModifier"`: The original C++ class name.
    #[serde(default = "HkbEventDrivenModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x7ed3f44e`: Unique value of this class.
    #[serde(default = "HkbEventDrivenModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbEventDrivenModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbEventDrivenModifierHkParam<'a>>
}

impl HkbEventDrivenModifier<'_> {
    /// Return `"hkbEventDrivenModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbEventDrivenModifier".into()
    }

    /// Return `"0x7ed3f44e"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x7ed3f44e".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEventDrivenModifierHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"activateEventId"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "activateEventId")]
    ActivateEventId(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"deactivateEventId"`
    /// -   type: `hkInt32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "deactivateEventId")]
    DeactivateEventId(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"activeByDefault"`
    /// -   type: `hkBool`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "activeByDefault")]
    ActiveByDefault(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"isActive"`
    /// -   type: `hkBool`
    /// - offset: 57
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isActive", skip_serializing)]
    IsActive(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbEventDrivenModifierHkParam<'de>, "@name",
    ("activateEventId" => ActivateEventId(Primitive<i32>)),
    ("deactivateEventId" => DeactivateEventId(Primitive<i32>)),
    ("activeByDefault" => ActiveByDefault(Primitive<bool>)),
    ("isActive" => IsActive(Primitive<bool>)),
}
