//! A Rust structure that implements a serializer/deserializer corresponding to `BGSGamebryoSequenceGenerator`, a class defined in C++
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
/// -    size: 72
/// -  vtable: true
/// -  parent: hkbGenerator/`d68aefc`(Non prefix hex signature)
/// - version: 2
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BgsGamebryoSequenceGenerator<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BGSGamebryoSequenceGenerator"`: The original C++ class name.
    #[serde(default = "BgsGamebryoSequenceGenerator::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xc8df2d77`: Unique value of this class.
    #[serde(default = "BgsGamebryoSequenceGenerator::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BgsGamebryoSequenceGeneratorHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BgsGamebryoSequenceGeneratorHkParam<'a>>
}

impl BgsGamebryoSequenceGenerator<'_> {
    /// Return `"BGSGamebryoSequenceGenerator"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BGSGamebryoSequenceGenerator".into()
    }

    /// Return `"0xc8df2d77"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xc8df2d77".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BgsGamebryoSequenceGeneratorHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"pSequence"`
    /// -   type: `char*`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "pSequence")]
    PSequence(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"eBlendModeFunction"`
    /// -   type: `enum BlendModeFunction`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eBlendModeFunction")]
    EBlendModeFunction(BlendModeFunction),
    /// # Field information in the original C++ class
    /// -   name:`"fPercent"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fPercent")]
    FPercent(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"events"`
    /// -   type: `hkArray&lt;void&gt;`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "events", skip_serializing)]
    Events(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"fTime"`
    /// -   type: `hkReal`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "fTime", skip_serializing)]
    FTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"bDelayedActivate"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bDelayedActivate", skip_serializing)]
    BDelayedActivate(Primitive<bool>),
    /// # Field information in the original C++ class
    /// -   name:`"bLooping"`
    /// -   type: `hkBool`
    /// - offset: 69
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "bLooping", skip_serializing)]
    BLooping(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BgsGamebryoSequenceGeneratorHkParam<'de>, "@name",
    ("pSequence" => PSequence(Primitive<Cow<'a, str>>)),
    ("eBlendModeFunction" => EBlendModeFunction(BlendModeFunction)),
    ("fPercent" => FPercent(Primitive<f32>)),
    ("events" => Events(Vec<()>)),
    ("fTime" => FTime(Primitive<f32>)),
    ("bDelayedActivate" => BDelayedActivate(Primitive<bool>)),
    ("bLooping" => BLooping(Primitive<bool>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BlendModeFunction {
    #[serde(rename = "BMF_NONE")]
    BmfNone = 0,
    #[serde(rename = "BMF_PERCENT")]
    BmfPercent = 1,
    #[serde(rename = "BMF_ONE_MINUS_PERCENT")]
    BmfOneMinusPercent = 2,
}
