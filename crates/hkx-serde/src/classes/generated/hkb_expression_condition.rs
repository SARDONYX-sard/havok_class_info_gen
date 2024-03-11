//! A Rust structure that implements a serializer/deserializer corresponding to `hkbExpressionCondition`, a class defined in C++
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
/// -    size: 16
/// -  vtable: true
/// -  parent: hkbCondition/`da8c7d7d`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbExpressionCondition<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbExpressionCondition"`: The original C++ class name.
    #[serde(default = "HkbExpressionCondition::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x1c3c1045`: Unique value of this class.
    #[serde(default = "HkbExpressionCondition::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbExpressionConditionHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbExpressionConditionHkParam<'a>>
}

impl HkbExpressionCondition<'_> {
    /// Return `"hkbExpressionCondition"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbExpressionCondition".into()
    }

    /// Return `"0x1c3c1045"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x1c3c1045".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbExpressionConditionHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"expression"`
    /// -   type: `hkStringPtr`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expression")]
    Expression(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"compiledExpressionSet"`
    /// -   type: `void*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "compiledExpressionSet", skip_serializing)]
    CompiledExpressionSet(()),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbExpressionConditionHkParam<'de>, "@name",
    ("expression" => Expression(Primitive<Cow<'a, str>>)),
    ("compiledExpressionSet" => CompiledExpressionSet(())),
}
