//! A Rust structure that implements a serializer/deserializer corresponding to `hkbEvaluateExpressionModifier`, a class defined in C++
//!
//! # NOTE
//! This file is generated automatically by parsing the rpt files obtained by executing the `hkxcmd Report` command.
use super::*;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// In XML, it is enclosed in a `hkobject` tag
/// and the `class` attribute contains the C++ class nam
///
/// # Information on the original C++ class
/// -    size: 64
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbEvaluateExpressionModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbEvaluateExpressionModifier"`: Name of this class.
    #[serde(default = "HkbEvaluateExpressionModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf900f6be`: Unique value of this class.
    #[serde(default = "HkbEvaluateExpressionModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbEvaluateExpressionModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbEvaluateExpressionModifierHkParam<'a>>
}

impl HkbEvaluateExpressionModifier<'_> {
    /// Return `"hkbEvaluateExpressionModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbEvaluateExpressionModifier".into()
    }

    /// Return `"0xf900f6be"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf900f6be".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEvaluateExpressionModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"expressions"`
    /// -   type: `struct hkbExpressionDataArray*`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "expressions")]
    Expressions(Box<HkbExpressionDataArray>),
    /// # Information on fields in the original C++ class
    /// -   name:`"compiledExpressionSet"`
    /// -   type: `void*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "compiledExpressionSet", skip_serializing)]
    CompiledExpressionSet(()),
    /// # Information on fields in the original C++ class
    /// -   name:`"internalExpressionsData"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "internalExpressionsData", skip_serializing)]
    InternalExpressionsData(Vec<()>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbEvaluateExpressionModifierHkParam<'de>, "@name",
    ("expressions" => Expressions(Box<HkbExpressionDataArray>)),
    ("compiledExpressionSet" => CompiledExpressionSet(())),
    ("internalExpressionsData" => InternalExpressionsData(Vec<()>)),
}
