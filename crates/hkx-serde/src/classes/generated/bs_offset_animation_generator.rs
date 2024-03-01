//! A Rust structure that implements a serializer/deserializer corresponding to `BSOffsetAnimationGenerator`, a class defined in C++
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
/// -    size: 128
/// -  vtable: true
/// -  parent: hkbGenerator/`d68aefc`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsOffsetAnimationGenerator<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSOffsetAnimationGenerator"`: Name of this class.
    #[serde(default = "BsOffsetAnimationGenerator::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xb8571122`: Unique value of this class.
    #[serde(default = "BsOffsetAnimationGenerator::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsOffsetAnimationGeneratorHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsOffsetAnimationGeneratorHkParam<'a>>
}

impl BsOffsetAnimationGenerator<'_> {
    /// Return `"BSOffsetAnimationGenerator"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BsOffsetAnimationGenerator".into()
    }

    /// Return `"0xb8571122"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xb8571122".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsOffsetAnimationGeneratorHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"pDefaultGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "pDefaultGenerator")]
    PDefaultGenerator(Box<HkbGenerator>),
    /// # Information on fields in the original C++ class
    /// -   name:`"pOffsetClipGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "pOffsetClipGenerator")]
    POffsetClipGenerator(Box<HkbGenerator>),
    /// # Information on fields in the original C++ class
    /// -   name:`"fOffsetVariable"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fOffsetVariable")]
    FOffsetVariable(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"fOffsetRangeStart"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fOffsetRangeStart")]
    FOffsetRangeStart(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"fOffsetRangeEnd"`
    /// -   type: `hkReal`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fOffsetRangeEnd")]
    FOffsetRangeEnd(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"BoneOffsetA"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "BoneOffsetA", skip_serializing)]
    BoneOffsetA(Vec<()>),
    /// # Information on fields in the original C++ class
    /// -   name:`"BoneIndexA"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "BoneIndexA", skip_serializing)]
    BoneIndexA(Vec<()>),
    /// # Information on fields in the original C++ class
    /// -   name:`"fCurrentPercentage"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "fCurrentPercentage", skip_serializing)]
    FCurrentPercentage(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"iCurrentFrame"`
    /// -   type: `hkUint32`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "iCurrentFrame", skip_serializing)]
    ICurrentFrame(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"bZeroOffset"`
    /// -   type: `hkBool`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bZeroOffset", skip_serializing)]
    BZeroOffset(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"bOffsetValid"`
    /// -   type: `hkBool`
    /// - offset: 113
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bOffsetValid", skip_serializing)]
    BOffsetValid(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsOffsetAnimationGeneratorHkParam<'de>, "@name",
    ("pDefaultGenerator" => PDefaultGenerator(Box<HkbGenerator>)),
    ("pOffsetClipGenerator" => POffsetClipGenerator(Box<HkbGenerator>)),
    ("fOffsetVariable" => FOffsetVariable(f64)),
    ("fOffsetRangeStart" => FOffsetRangeStart(f64)),
    ("fOffsetRangeEnd" => FOffsetRangeEnd(f64)),
    ("BoneOffsetA" => BoneOffsetA(Vec<()>)),
    ("BoneIndexA" => BoneIndexA(Vec<()>)),
    ("fCurrentPercentage" => FCurrentPercentage(f64)),
    ("iCurrentFrame" => ICurrentFrame(u32)),
    ("bZeroOffset" => BZeroOffset(bool)),
    ("bOffsetValid" => BOffsetValid(bool)),
}
