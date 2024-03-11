//! A Rust structure that implements a serializer/deserializer corresponding to `hkbCompiledExpressionSet`, a class defined in C++
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
/// -    size: 36
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbCompiledExpressionSet<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbCompiledExpressionSet"`: The original C++ class name.
    #[serde(default = "HkbCompiledExpressionSet::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x3a7d76cc`: Unique value of this class.
    #[serde(default = "HkbCompiledExpressionSet::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbCompiledExpressionSetHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbCompiledExpressionSetHkParam<'a>>
}

impl HkbCompiledExpressionSet<'_> {
    /// Return `"hkbCompiledExpressionSet"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbCompiledExpressionSet".into()
    }

    /// Return `"0x3a7d76cc"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x3a7d76cc".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCompiledExpressionSetHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"rpn"`
    /// -   type: `hkArray&lt;struct hkbCompiledExpressionSetToken&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rpn")]
    Rpn(Vec<HkbCompiledExpressionSetToken>),
    /// # Field information in the original C++ class
    /// -   name:`"expressionToRpnIndex"`
    /// -   type: `hkArray&lt;hkInt32&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expressionToRpnIndex")]
    ExpressionToRpnIndex(Vec<Primitive<i32>>),
    /// # Field information in the original C++ class
    /// -   name:`"numExpressions"`
    /// -   type: `hkInt8`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numExpressions")]
    NumExpressions(Primitive<i8>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbCompiledExpressionSetHkParam<'de>, "@name",
    ("rpn" => Rpn(Vec<HkbCompiledExpressionSetToken>)),
    ("expressionToRpnIndex" => ExpressionToRpnIndex(Vec<Primitive<i32>>)),
    ("numExpressions" => NumExpressions(Primitive<i8>)),
}
