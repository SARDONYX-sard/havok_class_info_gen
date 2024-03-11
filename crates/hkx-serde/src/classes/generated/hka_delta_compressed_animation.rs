//! A Rust structure that implements a serializer/deserializer corresponding to `hkaDeltaCompressedAnimation`, a class defined in C++
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
/// -    size: 120
/// -  vtable: true
/// -  parent: hkaAnimation/`a6fa7e88`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaDeltaCompressedAnimation<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaDeltaCompressedAnimation"`: The original C++ class name.
    #[serde(default = "HkaDeltaCompressedAnimation::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x90a68d40`: Unique value of this class.
    #[serde(default = "HkaDeltaCompressedAnimation::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaDeltaCompressedAnimationHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaDeltaCompressedAnimationHkParam<'a>>
}

impl HkaDeltaCompressedAnimation<'_> {
    /// Return `"hkaDeltaCompressedAnimation"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkaDeltaCompressedAnimation".into()
    }

    /// Return `"0x90a68d40"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x90a68d40".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaDeltaCompressedAnimationHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"numberOfPoses"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numberOfPoses")]
    NumberOfPoses(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"blockSize"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockSize")]
    BlockSize(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"qFormat"`
    /// -   type: `struct hkaDeltaCompressedAnimationQuantizationFormat`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "qFormat")]
    QFormat(HkaDeltaCompressedAnimationQuantizationFormat),
    /// # Field information in the original C++ class
    /// -   name:`"quantizedDataIdx"`
    /// -   type: `hkUint32`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "quantizedDataIdx")]
    QuantizedDataIdx(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"quantizedDataSize"`
    /// -   type: `hkUint32`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "quantizedDataSize")]
    QuantizedDataSize(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"staticMaskIdx"`
    /// -   type: `hkUint32`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "staticMaskIdx")]
    StaticMaskIdx(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"staticMaskSize"`
    /// -   type: `hkUint32`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "staticMaskSize")]
    StaticMaskSize(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"staticDOFsIdx"`
    /// -   type: `hkUint32`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "staticDOFsIdx")]
    StaticDoFsIdx(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"staticDOFsSize"`
    /// -   type: `hkUint32`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "staticDOFsSize")]
    StaticDoFsSize(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"numStaticTransformDOFs"`
    /// -   type: `hkUint32`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numStaticTransformDOFs")]
    NumStaticTransformDoFs(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"numDynamicTransformDOFs"`
    /// -   type: `hkUint32`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numDynamicTransformDOFs")]
    NumDynamicTransformDoFs(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"totalBlockSize"`
    /// -   type: `hkUint32`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "totalBlockSize")]
    TotalBlockSize(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"lastBlockSize"`
    /// -   type: `hkUint32`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lastBlockSize")]
    LastBlockSize(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"dataBuffer"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dataBuffer")]
    DataBuffer(Vec<Primitive<u8>>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaDeltaCompressedAnimationHkParam<'de>, "@name",
    ("numberOfPoses" => NumberOfPoses(Primitive<i32>)),
    ("blockSize" => BlockSize(Primitive<i32>)),
    ("qFormat" => QFormat(HkaDeltaCompressedAnimationQuantizationFormat)),
    ("quantizedDataIdx" => QuantizedDataIdx(Primitive<u32>)),
    ("quantizedDataSize" => QuantizedDataSize(Primitive<u32>)),
    ("staticMaskIdx" => StaticMaskIdx(Primitive<u32>)),
    ("staticMaskSize" => StaticMaskSize(Primitive<u32>)),
    ("staticDOFsIdx" => StaticDoFsIdx(Primitive<u32>)),
    ("staticDOFsSize" => StaticDoFsSize(Primitive<u32>)),
    ("numStaticTransformDOFs" => NumStaticTransformDoFs(Primitive<u32>)),
    ("numDynamicTransformDOFs" => NumDynamicTransformDoFs(Primitive<u32>)),
    ("totalBlockSize" => TotalBlockSize(Primitive<u32>)),
    ("lastBlockSize" => LastBlockSize(Primitive<u32>)),
    ("dataBuffer" => DataBuffer(Vec<Primitive<u8>>)),
}
