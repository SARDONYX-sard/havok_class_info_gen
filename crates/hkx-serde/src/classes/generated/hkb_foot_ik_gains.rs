//! A Rust structure that implements a serializer/deserializer corresponding to `hkbFootIkGains`, a class defined in C++
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
/// -    size: 48
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbFootIkGains<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbFootIkGains"`: The original C++ class name.
    #[serde(default = "HkbFootIkGains::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xa681b7f0`: Unique value of this class.
    #[serde(default = "HkbFootIkGains::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbFootIkGainsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbFootIkGainsHkParam<'a>>
}

impl HkbFootIkGains<'_> {
    /// Return `"hkbFootIkGains"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbFootIkGains".into()
    }

    /// Return `"0xa681b7f0"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xa681b7f0".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbFootIkGainsHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"onOffGain"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "onOffGain")]
    OnOffGain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"groundAscendingGain"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "groundAscendingGain")]
    GroundAscendingGain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"groundDescendingGain"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "groundDescendingGain")]
    GroundDescendingGain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"footPlantedGain"`
    /// -   type: `hkReal`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footPlantedGain")]
    FootPlantedGain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"footRaisedGain"`
    /// -   type: `hkReal`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footRaisedGain")]
    FootRaisedGain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"footUnlockGain"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "footUnlockGain")]
    FootUnlockGain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"worldFromModelFeedbackGain"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "worldFromModelFeedbackGain")]
    WorldFromModelFeedbackGain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"errorUpDownBias"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "errorUpDownBias")]
    ErrorUpDownBias(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"alignWorldFromModelGain"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "alignWorldFromModelGain")]
    AlignWorldFromModelGain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"hipOrientationGain"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hipOrientationGain")]
    HipOrientationGain(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxKneeAngleDifference"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxKneeAngleDifference")]
    MaxKneeAngleDifference(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"ankleOrientationGain"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "ankleOrientationGain")]
    AnkleOrientationGain(Primitive<f32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbFootIkGainsHkParam<'de>, "@name",
    ("onOffGain" => OnOffGain(Primitive<f32>)),
    ("groundAscendingGain" => GroundAscendingGain(Primitive<f32>)),
    ("groundDescendingGain" => GroundDescendingGain(Primitive<f32>)),
    ("footPlantedGain" => FootPlantedGain(Primitive<f32>)),
    ("footRaisedGain" => FootRaisedGain(Primitive<f32>)),
    ("footUnlockGain" => FootUnlockGain(Primitive<f32>)),
    ("worldFromModelFeedbackGain" => WorldFromModelFeedbackGain(Primitive<f32>)),
    ("errorUpDownBias" => ErrorUpDownBias(Primitive<f32>)),
    ("alignWorldFromModelGain" => AlignWorldFromModelGain(Primitive<f32>)),
    ("hipOrientationGain" => HipOrientationGain(Primitive<f32>)),
    ("maxKneeAngleDifference" => MaxKneeAngleDifference(Primitive<f32>)),
    ("ankleOrientationGain" => AnkleOrientationGain(Primitive<f32>)),
}
