//! A Rust structure that implements a serializer/deserializer corresponding to `hkpSimulation`, a class defined in C++
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
/// -    size: 44
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpSimulation<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpSimulation"`: The original C++ class name.
    #[serde(default = "HkpSimulation::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x97aba922`: Unique value of this class.
    #[serde(default = "HkpSimulation::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpSimulationHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpSimulationHkParam<'a>>
}

impl HkpSimulation<'_> {
    /// Return `"hkpSimulation"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpSimulation".into()
    }

    /// Return `"0x97aba922"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x97aba922".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpSimulationHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"determinismCheckFrameCounter"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "determinismCheckFrameCounter")]
    DeterminismCheckFrameCounter(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"world"`
    /// -   type: `struct hkpWorld*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "world")]
    World(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"lastProcessingStep"`
    /// -   type: `enum LastProcessingStep`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "lastProcessingStep")]
    LastProcessingStep(LastProcessingStep),
    /// # Field information in the original C++ class
    /// -   name:`"currentTime"`
    /// -   type: `hkReal`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentTime")]
    CurrentTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"currentPsiTime"`
    /// -   type: `hkReal`
    /// - offset: 24
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "currentPsiTime")]
    CurrentPsiTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"physicsDeltaTime"`
    /// -   type: `hkReal`
    /// - offset: 28
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "physicsDeltaTime")]
    PhysicsDeltaTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"simulateUntilTime"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "simulateUntilTime")]
    SimulateUntilTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"frameMarkerPsiSnap"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "frameMarkerPsiSnap")]
    FrameMarkerPsiSnap(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"previousStepResult"`
    /// -   type: `hkUint32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "previousStepResult")]
    PreviousStepResult(Primitive<u32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpSimulationHkParam<'de>, "@name",
    ("determinismCheckFrameCounter" => DeterminismCheckFrameCounter(Primitive<u32>)),
    ("world" => World(Cow<'a, str>)),
    ("lastProcessingStep" => LastProcessingStep(LastProcessingStep)),
    ("currentTime" => CurrentTime(Primitive<f32>)),
    ("currentPsiTime" => CurrentPsiTime(Primitive<f32>)),
    ("physicsDeltaTime" => PhysicsDeltaTime(Primitive<f32>)),
    ("simulateUntilTime" => SimulateUntilTime(Primitive<f32>)),
    ("frameMarkerPsiSnap" => FrameMarkerPsiSnap(Primitive<f32>)),
    ("previousStepResult" => PreviousStepResult(Primitive<u32>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FindContacts {
    #[serde(rename = "FIND_CONTACTS_DEFAULT")]
    FindContactsDefault = 0,
    #[serde(rename = "FIND_CONTACTS_EXTRA")]
    FindContactsExtra = 1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResetCollisionInformation {
    #[serde(rename = "RESET_TOI")]
    ResetToi = 1,
    #[serde(rename = "RESET_TIM")]
    ResetTim = 2,
    #[serde(rename = "RESET_AABB")]
    ResetAabb = 4,
    #[serde(rename = "RESET_ALL")]
    ResetAll = 7,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LastProcessingStep {
    #[serde(rename = "INTEGRATE")]
    Integrate = 0,
    #[serde(rename = "COLLIDE")]
    Collide = 1,
}
