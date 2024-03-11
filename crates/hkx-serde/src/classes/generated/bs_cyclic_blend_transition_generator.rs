//! A Rust structure that implements a serializer/deserializer corresponding to `BSCyclicBlendTransitionGenerator`, a class defined in C++
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
/// -    size: 112
/// -  vtable: true
/// -  parent: hkbGenerator/`d68aefc`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct BsCyclicBlendTransitionGenerator<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"BSCyclicBlendTransitionGenerator"`: The original C++ class name.
    #[serde(default = "BsCyclicBlendTransitionGenerator::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x5119eb06`: Unique value of this class.
    #[serde(default = "BsCyclicBlendTransitionGenerator::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<BsCyclicBlendTransitionGeneratorHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<BsCyclicBlendTransitionGeneratorHkParam<'a>>
}

impl BsCyclicBlendTransitionGenerator<'_> {
    /// Return `"BSCyclicBlendTransitionGenerator"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "BSCyclicBlendTransitionGenerator".into()
    }

    /// Return `"0x5119eb06"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x5119eb06".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum BsCyclicBlendTransitionGeneratorHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"pBlenderGenerator"`
    /// -   type: `struct hkbGenerator*`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE | ALIGN16`
    #[serde(rename = "pBlenderGenerator")]
    PBlenderGenerator(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"EventToFreezeBlendValue"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "EventToFreezeBlendValue")]
    EventToFreezeBlendValue(HkbEventProperty),
    /// # Field information in the original C++ class
    /// -   name:`"EventToCrossBlend"`
    /// -   type: `struct hkbEventProperty`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "EventToCrossBlend")]
    EventToCrossBlend(HkbEventProperty),
    /// # Field information in the original C++ class
    /// -   name:`"fBlendParameter"`
    /// -   type: `hkReal`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fBlendParameter")]
    FBlendParameter(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"fTransitionDuration"`
    /// -   type: `hkReal`
    /// - offset: 72
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "fTransitionDuration")]
    FTransitionDuration(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"eBlendCurve"`
    /// -   type: `enum BlendCurve`
    /// - offset: 76
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "eBlendCurve")]
    EBlendCurve(BlendCurve),
    /// # Field information in the original C++ class
    /// -   name:`"pTransitionBlenderGenerator"`
    /// -   type: `void*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | ALIGN16 | SERIALIZE_IGNORED`
    #[serde(rename = "pTransitionBlenderGenerator", skip_serializing)]
    PTransitionBlenderGenerator(()),
    /// # Field information in the original C++ class
    /// -   name:`"pTransitionEffect"`
    /// -   type: `void*`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE | ALIGN16 | SERIALIZE_IGNORED`
    #[serde(rename = "pTransitionEffect", skip_serializing)]
    PTransitionEffect(()),
    /// # Field information in the original C++ class
    /// -   name:`"currentMode"`
    /// -   type: `enum unknown`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "currentMode", skip_serializing)]
    CurrentMode(Unknown),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    BsCyclicBlendTransitionGeneratorHkParam<'de>, "@name",
    ("pBlenderGenerator" => PBlenderGenerator(Cow<'a, str>)),
    ("EventToFreezeBlendValue" => EventToFreezeBlendValue(HkbEventProperty)),
    ("EventToCrossBlend" => EventToCrossBlend(HkbEventProperty)),
    ("fBlendParameter" => FBlendParameter(Primitive<f32>)),
    ("fTransitionDuration" => FTransitionDuration(Primitive<f32>)),
    ("eBlendCurve" => EBlendCurve(BlendCurve)),
    ("pTransitionBlenderGenerator" => PTransitionBlenderGenerator(())),
    ("pTransitionEffect" => PTransitionEffect(())),
    ("currentMode" => CurrentMode(Unknown)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CurrentBlendMode {
    #[serde(rename = "MODE_INACTIVE")]
    ModeInactive = -1,
    #[serde(rename = "MODE_DEFAULT")]
    ModeDefault = 0,
    #[serde(rename = "MODE_FROZEN")]
    ModeFrozen = 1,
    #[serde(rename = "MODE_BLENDING")]
    ModeBlending = 2,
    #[serde(rename = "MODE_WAITINGFORBLENDING")]
    ModeWaitingforblending = 3,
}
