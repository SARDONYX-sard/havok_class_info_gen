//! A Rust structure that implements a serializer/deserializer corresponding to `hkbEvaluateExpressionModifierInternalState`, a class defined in C++
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
/// -    size: 20
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbEvaluateExpressionModifierInternalState<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbEvaluateExpressionModifierInternalState"`: Name of this class.
    #[serde(default = "HkbEvaluateExpressionModifierInternalState::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xb414d58e`: Unique value of this class.
    #[serde(default = "HkbEvaluateExpressionModifierInternalState::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbEvaluateExpressionModifierInternalStateHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbEvaluateExpressionModifierInternalStateHkParam<'a>>
}

impl HkbEvaluateExpressionModifierInternalState<'_> {
    /// Return `"hkbEvaluateExpressionModifierInternalState"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbEvaluateExpressionModifierInternalState".into()
    }

    /// Return `"0xb414d58e"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xb414d58e".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbEvaluateExpressionModifierInternalStateHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"internalExpressionsData"`
    /// -   type: `hkArray&lt;struct hkbEvaluateExpressionModifierInternalExpressionData&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "internalExpressionsData")]
    InternalExpressionsData(Vec<HkbEvaluateExpressionModifierInternalExpressionData>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbEvaluateExpressionModifierInternalStateHkParam<'de>, "@name",
    ("internalExpressionsData" => InternalExpressionsData(Vec<HkbEvaluateExpressionModifierInternalExpressionData>)),
}
