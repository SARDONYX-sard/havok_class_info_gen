//! A Rust structure that implements a serializer/deserializer corresponding to `hkaWaveletCompressedAnimationQuantizationFormat`, a class defined in C++
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
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaWaveletCompressedAnimationQuantizationFormat<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaWaveletCompressedAnimationQuantizationFormat"`: Name of this class.
    #[serde(default = "HkaWaveletCompressedAnimationQuantizationFormat::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x724a7561`: Unique value of this class.
    #[serde(default = "HkaWaveletCompressedAnimationQuantizationFormat::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaWaveletCompressedAnimationQuantizationFormatHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaWaveletCompressedAnimationQuantizationFormatHkParam<'a>>
}

impl HkaWaveletCompressedAnimationQuantizationFormat<'_> {
    /// Return `"hkaWaveletCompressedAnimationQuantizationFormat"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkaWaveletCompressedAnimationQuantizationFormat".into()
    }

    /// Return `"0x724a7561"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x724a7561".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaWaveletCompressedAnimationQuantizationFormatHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"maxBitWidth"`
    /// -   type: `hkUint8`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxBitWidth")]
    MaxBitWidth(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"preserved"`
    /// -   type: `hkUint8`
    /// - offset: 1
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "preserved")]
    Preserved(u8),
    /// # Information on fields in the original C++ class
    /// -   name:`"numD"`
    /// -   type: `hkUint32`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numD")]
    NumD(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"offsetIdx"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "offsetIdx")]
    OffsetIdx(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"scaleIdx"`
    /// -   type: `hkUint32`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "scaleIdx")]
    ScaleIdx(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"bitWidthIdx"`
    /// -   type: `hkUint32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bitWidthIdx")]
    BitWidthIdx(u32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaWaveletCompressedAnimationQuantizationFormatHkParam<'de>, "@name",
    ("maxBitWidth" => MaxBitWidth(u8)),
    ("preserved" => Preserved(u8)),
    ("numD" => NumD(u32)),
    ("offsetIdx" => OffsetIdx(u32)),
    ("scaleIdx" => ScaleIdx(u32)),
    ("bitWidthIdx" => BitWidthIdx(u32)),
}