//! A Rust structure that implements a serializer/deserializer corresponding to `hkaQuantizedAnimationTrackCompressionParams`, a class defined in C++
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
/// -    size: 16
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaQuantizedAnimationTrackCompressionParams<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaQuantizedAnimationTrackCompressionParams"`: Name of this class.
    #[serde(default = "HkaQuantizedAnimationTrackCompressionParams::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf7d64649`: Unique value of this class.
    #[serde(default = "HkaQuantizedAnimationTrackCompressionParams::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaQuantizedAnimationTrackCompressionParamsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaQuantizedAnimationTrackCompressionParamsHkParam<'a>>
}

impl HkaQuantizedAnimationTrackCompressionParams<'_> {
    /// Return `"hkaQuantizedAnimationTrackCompressionParams"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkaQuantizedAnimationTrackCompressionParams".into()
    }

    /// Return `"0xf7d64649"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf7d64649".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaQuantizedAnimationTrackCompressionParamsHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"rotationTolerance"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationTolerance")]
    RotationTolerance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"translationTolerance"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translationTolerance")]
    TranslationTolerance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"scaleTolerance"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scaleTolerance")]
    ScaleTolerance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"floatingTolerance"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatingTolerance")]
    FloatingTolerance(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaQuantizedAnimationTrackCompressionParamsHkParam<'de>, "@name",
    ("rotationTolerance" => RotationTolerance(f64)),
    ("translationTolerance" => TranslationTolerance(f64)),
    ("scaleTolerance" => ScaleTolerance(f64)),
    ("floatingTolerance" => FloatingTolerance(f64)),
}
