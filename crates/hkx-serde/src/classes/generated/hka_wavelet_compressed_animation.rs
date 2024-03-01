//! A Rust structure that implements a serializer/deserializer corresponding to `hkaWaveletCompressedAnimation`, a class defined in C++
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
/// -    size: 112
/// -  vtable: true
/// -  parent: hkaAnimation/`a6fa7e88`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaWaveletCompressedAnimation<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaWaveletCompressedAnimation"`: Name of this class.
    #[serde(default = "HkaWaveletCompressedAnimation::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x77cf0962`: Unique value of this class.
    #[serde(default = "HkaWaveletCompressedAnimation::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaWaveletCompressedAnimationHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaWaveletCompressedAnimationHkParam<'a>>
}

impl HkaWaveletCompressedAnimation<'_> {
    /// Return `"hkaWaveletCompressedAnimation"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkaWaveletCompressedAnimation".into()
    }

    /// Return `"0x77cf0962"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x77cf0962".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaWaveletCompressedAnimationHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"numberOfPoses"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numberOfPoses")]
    NumberOfPoses(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"blockSize"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockSize")]
    BlockSize(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"qFormat"`
    /// -   type: `struct hkaWaveletCompressedAnimationQuantizationFormat`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "qFormat")]
    QFormat(HkaWaveletCompressedAnimationQuantizationFormat),
    /// # Information on fields in the original C++ class
    /// -   name:`"staticMaskIdx"`
    /// -   type: `hkUint32`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "staticMaskIdx")]
    StaticMaskIdx(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"staticDOFsIdx"`
    /// -   type: `hkUint32`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "staticDOFsIdx")]
    StaticDoFsIdx(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"numStaticTransformDOFs"`
    /// -   type: `hkUint32`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numStaticTransformDOFs")]
    NumStaticTransformDoFs(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"numDynamicTransformDOFs"`
    /// -   type: `hkUint32`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numDynamicTransformDOFs")]
    NumDynamicTransformDoFs(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"blockIndexIdx"`
    /// -   type: `hkUint32`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockIndexIdx")]
    BlockIndexIdx(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"blockIndexSize"`
    /// -   type: `hkUint32`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockIndexSize")]
    BlockIndexSize(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"quantizedDataIdx"`
    /// -   type: `hkUint32`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "quantizedDataIdx")]
    QuantizedDataIdx(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"quantizedDataSize"`
    /// -   type: `hkUint32`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "quantizedDataSize")]
    QuantizedDataSize(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"dataBuffer"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dataBuffer")]
    DataBuffer(Vec<u8>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaWaveletCompressedAnimationHkParam<'de>, "@name",
    ("numberOfPoses" => NumberOfPoses(i32)),
    ("blockSize" => BlockSize(i32)),
    ("qFormat" => QFormat(HkaWaveletCompressedAnimationQuantizationFormat)),
    ("staticMaskIdx" => StaticMaskIdx(u32)),
    ("staticDOFsIdx" => StaticDoFsIdx(u32)),
    ("numStaticTransformDOFs" => NumStaticTransformDoFs(u32)),
    ("numDynamicTransformDOFs" => NumDynamicTransformDoFs(u32)),
    ("blockIndexIdx" => BlockIndexIdx(u32)),
    ("blockIndexSize" => BlockIndexSize(u32)),
    ("quantizedDataIdx" => QuantizedDataIdx(u32)),
    ("quantizedDataSize" => QuantizedDataSize(u32)),
    ("dataBuffer" => DataBuffer(Vec<u8>)),
}
