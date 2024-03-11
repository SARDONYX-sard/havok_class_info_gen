//! A Rust structure that implements a serializer/deserializer corresponding to `hkbSetWordVariableCommand`, a class defined in C++
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
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbSetWordVariableCommand<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbSetWordVariableCommand"`: The original C++ class name.
    #[serde(default = "HkbSetWordVariableCommand::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf3ae5fca`: Unique value of this class.
    #[serde(default = "HkbSetWordVariableCommand::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbSetWordVariableCommandHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbSetWordVariableCommandHkParam<'a>>
}

impl HkbSetWordVariableCommand<'_> {
    /// Return `"hkbSetWordVariableCommand"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbSetWordVariableCommand".into()
    }

    /// Return `"0xf3ae5fca"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf3ae5fca".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSetWordVariableCommandHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"quadValue"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "quadValue")]
    QuadValue(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"characterId"`
    /// -   type: `hkUint64`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterId")]
    CharacterId(Primitive<u64>),
    /// # Field information in the original C++ class
    /// -   name:`"variableId"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "variableId")]
    VariableId(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"value"`
    /// -   type: `struct hkbVariableValue`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "value")]
    Value(HkbVariableValue),
    /// # Field information in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum VariableType`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(VariableType),
    /// # Field information in the original C++ class
    /// -   name:`"global"`
    /// -   type: `hkBool`
    /// - offset: 49
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "global")]
    Global(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbSetWordVariableCommandHkParam<'de>, "@name",
    ("quadValue" => QuadValue(Vector4<f32>)),
    ("characterId" => CharacterId(Primitive<u64>)),
    ("variableId" => VariableId(Primitive<i32>)),
    ("value" => Value(HkbVariableValue)),
    ("type" => Type(VariableType)),
    ("global" => Global(Primitive<bool>)),
}
