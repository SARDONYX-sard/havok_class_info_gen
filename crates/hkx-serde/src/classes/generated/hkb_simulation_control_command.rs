//! A Rust structure that implements a serializer/deserializer corresponding to `hkbSimulationControlCommand`, a class defined in C++
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
/// -    size: 12
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbSimulationControlCommand<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbSimulationControlCommand"`: Name of this class.
    #[serde(default = "HkbSimulationControlCommand::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x2a241367`: Unique value of this class.
    #[serde(default = "HkbSimulationControlCommand::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbSimulationControlCommandHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbSimulationControlCommandHkParam<'a>>
}

impl HkbSimulationControlCommand<'_> {
    /// Return `"hkbSimulationControlCommand"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbSimulationControlCommand".into()
    }

    /// Return `"0x2a241367"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x2a241367".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbSimulationControlCommandHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"command"`
    /// -   type: `enum SimulationControlCommand`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "command")]
    Command(SimulationControlCommand),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbSimulationControlCommandHkParam<'de>, "@name",
    ("command" => Command(SimulationControlCommand)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SimulationControlCommand {
    #[serde(rename = "COMMAND_PLAY")]
    CommandPlay = 0,
    #[serde(rename = "COMMAND_PAUSE")]
    CommandPause = 1,
    #[serde(rename = "COMMAND_STEP")]
    CommandStep = 2,
    #[serde(rename = "COMMAND_STOP")]
    CommandStop = 3,
    #[serde(rename = "COMMAND_ACCUMULATE_MOTION")]
    CommandAccumulateMotion = 4,
    #[serde(rename = "COMMAND_DO_NOT_ACCUMULATE_MOTION")]
    CommandDoNotAccumulateMotion = 5,
}
