//! A Rust structure that implements a serializer/deserializer corresponding to `hkaSplineCompressedAnimation`, a class defined in C++
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
/// -    size: 132
/// -  vtable: true
/// -  parent: hkaAnimation/`a6fa7e88`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaSplineCompressedAnimation<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaSplineCompressedAnimation"`: Name of this class.
    #[serde(default = "HkaSplineCompressedAnimation::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x792ee0bb`: Unique value of this class.
    #[serde(default = "HkaSplineCompressedAnimation::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaSplineCompressedAnimationHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaSplineCompressedAnimationHkParam<'a>>
}

impl HkaSplineCompressedAnimation<'_> {
    /// Return `"hkaSplineCompressedAnimation"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkaSplineCompressedAnimation".into()
    }

    /// Return `"0x792ee0bb"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x792ee0bb".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaSplineCompressedAnimationHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"numFrames"`
    /// -   type: `hkInt32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numFrames")]
    NumFrames(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"numBlocks"`
    /// -   type: `hkInt32`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numBlocks")]
    NumBlocks(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxFramesPerBlock"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxFramesPerBlock")]
    MaxFramesPerBlock(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"maskAndQuantizationSize"`
    /// -   type: `hkInt32`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maskAndQuantizationSize")]
    MaskAndQuantizationSize(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"blockDuration"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockDuration")]
    BlockDuration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"blockInverseDuration"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockInverseDuration")]
    BlockInverseDuration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"frameDuration"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frameDuration")]
    FrameDuration(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"blockOffsets"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "blockOffsets")]
    BlockOffsets(Vec<u32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"floatBlockOffsets"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatBlockOffsets")]
    FloatBlockOffsets(Vec<u32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"transformOffsets"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "transformOffsets")]
    TransformOffsets(Vec<u32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"floatOffsets"`
    /// -   type: `hkArray&lt;hkUint32&gt;`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "floatOffsets")]
    FloatOffsets(Vec<u32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"data"`
    /// -   type: `hkArray&lt;hkUint8&gt;`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "data")]
    Data(Vec<u8>),
    /// # Information on fields in the original C++ class
    /// -   name:`"endian"`
    /// -   type: `hkInt32`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "endian")]
    Endian(i32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaSplineCompressedAnimationHkParam<'de>, "@name",
    ("numFrames" => NumFrames(i32)),
    ("numBlocks" => NumBlocks(i32)),
    ("maxFramesPerBlock" => MaxFramesPerBlock(i32)),
    ("maskAndQuantizationSize" => MaskAndQuantizationSize(i32)),
    ("blockDuration" => BlockDuration(f64)),
    ("blockInverseDuration" => BlockInverseDuration(f64)),
    ("frameDuration" => FrameDuration(f64)),
    ("blockOffsets" => BlockOffsets(Vec<u32>)),
    ("floatBlockOffsets" => FloatBlockOffsets(Vec<u32>)),
    ("transformOffsets" => TransformOffsets(Vec<u32>)),
    ("floatOffsets" => FloatOffsets(Vec<u32>)),
    ("data" => Data(Vec<u8>)),
    ("endian" => Endian(i32)),
}
