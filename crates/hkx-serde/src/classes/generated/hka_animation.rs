//! A Rust structure that implements a serializer/deserializer corresponding to `hkaAnimation`, a class defined in C++
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
/// -    size: 40
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkaAnimation<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkaAnimation"`: The original C++ class name.
    #[serde(default = "HkaAnimation::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa6fa7e88`: Unique value of this class.
    #[serde(default = "HkaAnimation::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkaAnimationHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkaAnimationHkParam<'a>>
}

impl HkaAnimation<'_> {
    /// Return `"hkaAnimation"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkaAnimation".into()
    }

    /// Return `"0xa6fa7e88"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa6fa7e88".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkaAnimationHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum AnimationType`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "type")]
    Type(AnimationType),
    /// # Field information in the original C++ class
    /// -   name:`"duration"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "duration")]
    Duration(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"numberOfTransformTracks"`
    /// -   type: `hkInt32`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numberOfTransformTracks")]
    NumberOfTransformTracks(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"numberOfFloatTracks"`
    /// -   type: `hkInt32`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "numberOfFloatTracks")]
    NumberOfFloatTracks(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"extractedMotion"`
    /// -   type: `struct hkaAnimatedReferenceFrame*`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extractedMotion")]
    ExtractedMotion(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"annotationTracks"`
    /// -   type: `hkArray&lt;struct hkaAnnotationTrack&gt;`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "annotationTracks")]
    AnnotationTracks(Vec<HkaAnnotationTrack>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkaAnimationHkParam<'de>, "@name",
    ("type" => Type(AnimationType)),
    ("duration" => Duration(Primitive<f32>)),
    ("numberOfTransformTracks" => NumberOfTransformTracks(Primitive<i32>)),
    ("numberOfFloatTracks" => NumberOfFloatTracks(Primitive<i32>)),
    ("extractedMotion" => ExtractedMotion(Cow<'a, str>)),
    ("annotationTracks" => AnnotationTracks(Vec<HkaAnnotationTrack>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AnimationType {
    #[serde(rename = "HK_UNKNOWN_ANIMATION")]
    HkUnknownAnimation = 0,
    #[serde(rename = "HK_INTERLEAVED_ANIMATION")]
    HkInterleavedAnimation = 1,
    #[serde(rename = "HK_DELTA_COMPRESSED_ANIMATION")]
    HkDeltaCompressedAnimation = 2,
    #[serde(rename = "HK_WAVELET_COMPRESSED_ANIMATION")]
    HkWaveletCompressedAnimation = 3,
    #[serde(rename = "HK_MIRRORED_ANIMATION")]
    HkMirroredAnimation = 4,
    #[serde(rename = "HK_SPLINE_COMPRESSED_ANIMATION")]
    HkSplineCompressedAnimation = 5,
    #[serde(rename = "HK_QUANTIZED_COMPRESSED_ANIMATION")]
    HkQuantizedCompressedAnimation = 6,
}
