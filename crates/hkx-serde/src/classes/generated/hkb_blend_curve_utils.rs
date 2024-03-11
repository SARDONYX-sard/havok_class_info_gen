//! A Rust structure that implements a serializer/deserializer corresponding to `hkbBlendCurveUtils`, a class defined in C++
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
/// -    size: 1
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbBlendCurveUtils<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbBlendCurveUtils"`: The original C++ class name.
    #[serde(default = "HkbBlendCurveUtils::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x23041af0`: Unique value of this class.
    #[serde(default = "HkbBlendCurveUtils::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbBlendCurveUtilsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbBlendCurveUtilsHkParam<'a>>
}

impl HkbBlendCurveUtils<'_> {
    /// Return `"hkbBlendCurveUtils"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbBlendCurveUtils".into()
    }

    /// Return `"0x23041af0"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x23041af0".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbBlendCurveUtilsHkParam<'a> {
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbBlendCurveUtilsHkParam<'de>, "@name",
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BlendCurve {
    #[serde(rename = "BLEND_CURVE_SMOOTH")]
    BlendCurveSmooth = 0,
    #[serde(rename = "BLEND_CURVE_LINEAR")]
    BlendCurveLinear = 1,
    #[serde(rename = "BLEND_CURVE_LINEAR_TO_SMOOTH")]
    BlendCurveLinearToSmooth = 2,
    #[serde(rename = "BLEND_CURVE_SMOOTH_TO_LINEAR")]
    BlendCurveSmoothToLinear = 3,
}
