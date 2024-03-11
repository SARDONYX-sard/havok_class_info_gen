//! A Rust structure that implements a serializer/deserializer corresponding to `hkbWorldEnums`, a class defined in C++
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
pub struct HkbWorldEnums<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbWorldEnums"`: The original C++ class name.
    #[serde(default = "HkbWorldEnums::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x25640b46`: Unique value of this class.
    #[serde(default = "HkbWorldEnums::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbWorldEnumsHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbWorldEnumsHkParam<'a>>
}

impl HkbWorldEnums<'_> {
    /// Return `"hkbWorldEnums"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbWorldEnums".into()
    }

    /// Return `"0x25640b46"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x25640b46".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbWorldEnumsHkParam<'a> {
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbWorldEnumsHkParam<'de>, "@name",
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SimulationState {
    #[serde(rename = "SIMULATION_STATE_PLAY")]
    SimulationStatePlay = 0,
    #[serde(rename = "SIMULATION_STATE_PAUSE")]
    SimulationStatePause = 1,
    #[serde(rename = "SIMULATION_STATE_STEP")]
    SimulationStateStep = 2,
    #[serde(rename = "SIMULATION_STATE_STOP")]
    SimulationStateStop = 3,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AccumulateMotionState {
    #[serde(rename = "ACCUMULATE_MOTION")]
    AccumulateMotion = 0,
    #[serde(rename = "DO_NOT_ACCUMULATE_MOTION")]
    DoNotAccumulateMotion = 1,
}
