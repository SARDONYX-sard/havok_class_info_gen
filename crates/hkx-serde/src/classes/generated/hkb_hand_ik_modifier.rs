//! A Rust structure that implements a serializer/deserializer corresponding to `hkbHandIkModifier`, a class defined in C++
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
/// -    size: 72
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbHandIkModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbHandIkModifier"`: The original C++ class name.
    #[serde(default = "HkbHandIkModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xef8bc2f7`: Unique value of this class.
    #[serde(default = "HkbHandIkModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbHandIkModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbHandIkModifierHkParam<'a>>
}

impl HkbHandIkModifier<'_> {
    /// Return `"hkbHandIkModifier"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbHandIkModifier".into()
    }

    /// Return `"0xef8bc2f7"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xef8bc2f7".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbHandIkModifierHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"hands"`
    /// -   type: `hkArray&lt;struct hkbHandIkModifierHand&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hands")]
    Hands(Vec<HkbHandIkModifierHand>),
    /// # Field information in the original C++ class
    /// -   name:`"fadeInOutCurve"`
    /// -   type: `enum BlendCurve`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fadeInOutCurve")]
    FadeInOutCurve(BlendCurve),
    /// # Field information in the original C++ class
    /// -   name:`"internalHandData"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "internalHandData", skip_serializing)]
    InternalHandData(Vec<()>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandIkModifierHkParam<'de>, "@name",
    ("hands" => Hands(Vec<HkbHandIkModifierHand>)),
    ("fadeInOutCurve" => FadeInOutCurve(BlendCurve)),
    ("internalHandData" => InternalHandData(Vec<()>)),
}
