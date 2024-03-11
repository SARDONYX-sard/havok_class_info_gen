//! A Rust structure that implements a serializer/deserializer corresponding to `hkbBehaviorGraphStringData`, a class defined in C++
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
/// -    size: 56
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbBehaviorGraphStringData<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbBehaviorGraphStringData"`: The original C++ class name.
    #[serde(default = "HkbBehaviorGraphStringData::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xc713064e`: Unique value of this class.
    #[serde(default = "HkbBehaviorGraphStringData::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbBehaviorGraphStringDataHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbBehaviorGraphStringDataHkParam<'a>>
}

impl HkbBehaviorGraphStringData<'_> {
    /// Return `"hkbBehaviorGraphStringData"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbBehaviorGraphStringData".into()
    }

    /// Return `"0xc713064e"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc713064e".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBehaviorGraphStringDataHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"eventNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eventNames")]
    EventNames(Vec<Primitive<Cow<'a, str>>>),
    /// # Field information in the original C++ class
    /// -   name:`"attributeNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "attributeNames")]
    AttributeNames(Vec<Primitive<Cow<'a, str>>>),
    /// # Field information in the original C++ class
    /// -   name:`"variableNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableNames")]
    VariableNames(Vec<Primitive<Cow<'a, str>>>),
    /// # Field information in the original C++ class
    /// -   name:`"characterPropertyNames"`
    /// -   type: `hkArray&lt;hkStringPtr&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterPropertyNames")]
    CharacterPropertyNames(Vec<Primitive<Cow<'a, str>>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbBehaviorGraphStringDataHkParam<'de>, "@name",
    ("eventNames" => EventNames(Vec<Primitive<Cow<'a, str>>>)),
    ("attributeNames" => AttributeNames(Vec<Primitive<Cow<'a, str>>>)),
    ("variableNames" => VariableNames(Vec<Primitive<Cow<'a, str>>>)),
    ("characterPropertyNames" => CharacterPropertyNames(Vec<Primitive<Cow<'a, str>>>)),
}
