//! A Rust structure that implements a serializer/deserializer corresponding to `hkbVariableBindingSet`, a class defined in C++
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
/// -    size: 28
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbVariableBindingSet<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbVariableBindingSet"`: The original C++ class name.
    #[serde(default = "HkbVariableBindingSet::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x338ad4ff`: Unique value of this class.
    #[serde(default = "HkbVariableBindingSet::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbVariableBindingSetHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbVariableBindingSetHkParam<'a>>
}

impl HkbVariableBindingSet<'_> {
    /// Return `"hkbVariableBindingSet"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbVariableBindingSet".into()
    }

    /// Return `"0x338ad4ff"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x338ad4ff".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbVariableBindingSetHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"bindings"`
    /// -   type: `hkArray&lt;struct hkbVariableBindingSetBinding&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bindings")]
    Bindings(Vec<HkbVariableBindingSetBinding>),
    /// # Field information in the original C++ class
    /// -   name:`"indexOfBindingToEnable"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "indexOfBindingToEnable")]
    IndexOfBindingToEnable(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"hasOutputBinding"`
    /// -   type: `hkBool`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "hasOutputBinding", skip_serializing)]
    HasOutputBinding(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbVariableBindingSetHkParam<'de>, "@name",
    ("bindings" => Bindings(Vec<HkbVariableBindingSetBinding>)),
    ("indexOfBindingToEnable" => IndexOfBindingToEnable(Primitive<i32>)),
    ("hasOutputBinding" => HasOutputBinding(Primitive<bool>)),
}
