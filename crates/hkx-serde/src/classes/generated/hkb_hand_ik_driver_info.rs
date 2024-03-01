//! A Rust structure that implements a serializer/deserializer corresponding to `hkbHandIkDriverInfo`, a class defined in C++
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
/// -    size: 24
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbHandIkDriverInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbHandIkDriverInfo"`: Name of this class.
    #[serde(default = "HkbHandIkDriverInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xc299090a`: Unique value of this class.
    #[serde(default = "HkbHandIkDriverInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbHandIkDriverInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbHandIkDriverInfoHkParam<'a>>
}

impl HkbHandIkDriverInfo<'_> {
    /// Return `"hkbHandIkDriverInfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbHandIkDriverInfo".into()
    }

    /// Return `"0xc299090a"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc299090a".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbHandIkDriverInfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"hands"`
    /// -   type: `hkArray&lt;struct hkbHandIkDriverInfoHand&gt;`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hands")]
    Hands(Vec<HkbHandIkDriverInfoHand>),
    /// # Information on fields in the original C++ class
    /// -   name:`"fadeInOutCurve"`
    /// -   type: `enum BlendCurve`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fadeInOutCurve")]
    FadeInOutCurve(BlendCurve),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbHandIkDriverInfoHkParam<'de>, "@name",
    ("hands" => Hands(Vec<HkbHandIkDriverInfoHand>)),
    ("fadeInOutCurve" => FadeInOutCurve(BlendCurve)),
}
