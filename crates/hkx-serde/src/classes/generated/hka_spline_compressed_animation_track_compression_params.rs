//! A Rust structure that implements a serializer/deserializer corresponding to `hkaSplineCompressedAnimationTrackCompressionParams`, a class defined in C++
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
/// -    size: 28
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaSplineCompressedAnimationTrackCompressionParams<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaSplineCompressedAnimationTrackCompressionParams"`: Name of this class.
    #[serde(default = "HkaSplineCompressedAnimationTrackCompressionParams::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x42e878d3`: Unique value of this class.
    #[serde(default = "HkaSplineCompressedAnimationTrackCompressionParams::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaSplineCompressedAnimationTrackCompressionParamsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaSplineCompressedAnimationTrackCompressionParamsHkParam<'a>>
}

impl HkaSplineCompressedAnimationTrackCompressionParams<'_> {
    /// Return `"hkaSplineCompressedAnimationTrackCompressionParams"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkaSplineCompressedAnimationTrackCompressionParams".into()
    }

    /// Return `"0x42e878d3"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x42e878d3".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaSplineCompressedAnimationTrackCompressionParamsHkParam<'a> {
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
    /// # Information on fields in the original C++ class
    /// -   name:`"rotationDegree"`
    /// -   type: `hkUint16`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationDegree")]
    RotationDegree(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"translationDegree"`
    /// -   type: `hkUint16`
    /// - offset: 18
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translationDegree")]
    TranslationDegree(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"scaleDegree"`
    /// -   type: `hkUint16`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scaleDegree")]
    ScaleDegree(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"floatingDegree"`
    /// -   type: `hkUint16`
    /// - offset: 22
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatingDegree")]
    FloatingDegree(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"rotationQuantizationType"`
    /// -   type: `enum RotationQuantization`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationQuantizationType")]
    RotationQuantizationType(RotationQuantization),
    /// # Information on fields in the original C++ class
    /// -   name:`"translationQuantizationType"`
    /// -   type: `enum ScalarQuantization`
    /// - offset: 25
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "translationQuantizationType")]
    TranslationQuantizationType(ScalarQuantization),
    /// # Information on fields in the original C++ class
    /// -   name:`"scaleQuantizationType"`
    /// -   type: `enum ScalarQuantization`
    /// - offset: 26
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scaleQuantizationType")]
    ScaleQuantizationType(ScalarQuantization),
    /// # Information on fields in the original C++ class
    /// -   name:`"floatQuantizationType"`
    /// -   type: `enum ScalarQuantization`
    /// - offset: 27
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatQuantizationType")]
    FloatQuantizationType(ScalarQuantization),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaSplineCompressedAnimationTrackCompressionParamsHkParam<'de>, "@name",
    ("rotationTolerance" => RotationTolerance(f64)),
    ("translationTolerance" => TranslationTolerance(f64)),
    ("scaleTolerance" => ScaleTolerance(f64)),
    ("floatingTolerance" => FloatingTolerance(f64)),
    ("rotationDegree" => RotationDegree(u16)),
    ("translationDegree" => TranslationDegree(u16)),
    ("scaleDegree" => ScaleDegree(u16)),
    ("floatingDegree" => FloatingDegree(u16)),
    ("rotationQuantizationType" => RotationQuantizationType(RotationQuantization)),
    ("translationQuantizationType" => TranslationQuantizationType(ScalarQuantization)),
    ("scaleQuantizationType" => ScaleQuantizationType(ScalarQuantization)),
    ("floatQuantizationType" => FloatQuantizationType(ScalarQuantization)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RotationQuantization {
    #[serde(rename = "POLAR32")]
    Polar32 = 0,
    #[serde(rename = "THREECOMP40")]
    Threecomp40 = 1,
    #[serde(rename = "THREECOMP48")]
    Threecomp48 = 2,
    #[serde(rename = "THREECOMP24")]
    Threecomp24 = 3,
    #[serde(rename = "STRAIGHT16")]
    Straight16 = 4,
    #[serde(rename = "UNCOMPRESSED")]
    Uncompressed = 5,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ScalarQuantization {
    #[serde(rename = "BITS8")]
    Bits8 = 0,
    #[serde(rename = "BITS16")]
    Bits16 = 1,
}
