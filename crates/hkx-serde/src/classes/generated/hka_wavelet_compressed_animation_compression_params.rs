//! A Rust structure that implements a serializer/deserializer corresponding to `hkaWaveletCompressedAnimationCompressionParams`, a class defined in C++
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
/// -    size: 36
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaWaveletCompressedAnimationCompressionParams<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaWaveletCompressedAnimationCompressionParams"`: Name of this class.
    #[serde(default = "HkaWaveletCompressedAnimationCompressionParams::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x27c6cafa`: Unique value of this class.
    #[serde(default = "HkaWaveletCompressedAnimationCompressionParams::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaWaveletCompressedAnimationCompressionParamsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaWaveletCompressedAnimationCompressionParamsHkParam<'a>>
}

impl HkaWaveletCompressedAnimationCompressionParams<'_> {
    /// Return `"hkaWaveletCompressedAnimationCompressionParams"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkaWaveletCompressedAnimationCompressionParams".into()
    }

    /// Return `"0x27c6cafa"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x27c6cafa".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaWaveletCompressedAnimationCompressionParamsHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"quantizationBits"`
    /// -   type: `hkUint16`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "quantizationBits")]
    QuantizationBits(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"blockSize"`
    /// -   type: `hkUint16`
    /// - offset: 2
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockSize")]
    BlockSize(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"preserve"`
    /// -   type: `hkUint16`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "preserve")]
    Preserve(u16),
    /// # Information on fields in the original C++ class
    /// -   name:`"truncProp"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "truncProp")]
    TruncProp(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"useOldStyleTruncation"`
    /// -   type: `hkBool`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "useOldStyleTruncation")]
    UseOldStyleTruncation(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"absolutePositionTolerance"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "absolutePositionTolerance")]
    AbsolutePositionTolerance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"relativePositionTolerance"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "relativePositionTolerance")]
    RelativePositionTolerance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"rotationTolerance"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotationTolerance")]
    RotationTolerance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"scaleTolerance"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scaleTolerance")]
    ScaleTolerance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"absoluteFloatTolerance"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "absoluteFloatTolerance")]
    AbsoluteFloatTolerance(f64),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaWaveletCompressedAnimationCompressionParamsHkParam<'de>, "@name",
    ("quantizationBits" => QuantizationBits(u16)),
    ("blockSize" => BlockSize(u16)),
    ("preserve" => Preserve(u16)),
    ("truncProp" => TruncProp(f64)),
    ("useOldStyleTruncation" => UseOldStyleTruncation(bool)),
    ("absolutePositionTolerance" => AbsolutePositionTolerance(f64)),
    ("relativePositionTolerance" => RelativePositionTolerance(f64)),
    ("rotationTolerance" => RotationTolerance(f64)),
    ("scaleTolerance" => ScaleTolerance(f64)),
    ("absoluteFloatTolerance" => AbsoluteFloatTolerance(f64)),
}
