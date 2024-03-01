//! A Rust structure that implements a serializer/deserializer corresponding to `hkbCharacterControllerModifier`, a class defined in C++
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
/// -    size: 144
/// -  vtable: true
/// -  parent: hkbModifier/`96ec5ced`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbCharacterControllerModifier<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbCharacterControllerModifier"`: Name of this class.
    #[serde(default = "HkbCharacterControllerModifier::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xf675d6fb`: Unique value of this class.
    #[serde(default = "HkbCharacterControllerModifier::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbCharacterControllerModifierHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbCharacterControllerModifierHkParam<'a>>
}

impl HkbCharacterControllerModifier<'_> {
    /// Return `"hkbCharacterControllerModifier"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkbCharacterControllerModifier".into()
    }

    /// Return `"0xf675d6fb"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xf675d6fb".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbCharacterControllerModifierHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"controlData"`
    /// -   type: `struct hkbCharacterControllerControlData`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "controlData")]
    ControlData(HkbCharacterControllerControlData),
    /// # Information on fields in the original C++ class
    /// -   name:`"initialVelocity"`
    /// -   type: `hkVector4`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initialVelocity")]
    InitialVelocity(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"initialVelocityCoordinates"`
    /// -   type: `enum InitialVelocityCoordinates`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "initialVelocityCoordinates")]
    InitialVelocityCoordinates(InitialVelocityCoordinates),
    /// # Information on fields in the original C++ class
    /// -   name:`"motionMode"`
    /// -   type: `enum MotionMode`
    /// - offset: 97
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "motionMode")]
    MotionMode(MotionMode),
    /// # Information on fields in the original C++ class
    /// -   name:`"forceDownwardMomentum"`
    /// -   type: `hkBool`
    /// - offset: 98
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "forceDownwardMomentum")]
    ForceDownwardMomentum(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"applyGravity"`
    /// -   type: `hkBool`
    /// - offset: 99
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "applyGravity")]
    ApplyGravity(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"setInitialVelocity"`
    /// -   type: `hkBool`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "setInitialVelocity")]
    SetInitialVelocity(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"isTouchingGround"`
    /// -   type: `hkBool`
    /// - offset: 101
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "isTouchingGround")]
    IsTouchingGround(bool),
    /// # Information on fields in the original C++ class
    /// -   name:`"gravity"`
    /// -   type: `hkVector4`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "gravity", skip_serializing)]
    Gravity(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"timestep"`
    /// -   type: `hkReal`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "timestep", skip_serializing)]
    Timestep(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"isInitialVelocityAdded"`
    /// -   type: `hkBool`
    /// - offset: 132
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "isInitialVelocityAdded", skip_serializing)]
    IsInitialVelocityAdded(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbCharacterControllerModifierHkParam<'de>, "@name",
    ("controlData" => ControlData(HkbCharacterControllerControlData)),
    ("initialVelocity" => InitialVelocity(cgmath::Vector4<f32>)),
    ("initialVelocityCoordinates" => InitialVelocityCoordinates(InitialVelocityCoordinates)),
    ("motionMode" => MotionMode(MotionMode)),
    ("forceDownwardMomentum" => ForceDownwardMomentum(bool)),
    ("applyGravity" => ApplyGravity(bool)),
    ("setInitialVelocity" => SetInitialVelocity(bool)),
    ("isTouchingGround" => IsTouchingGround(bool)),
    ("gravity" => Gravity(cgmath::Vector4<f32>)),
    ("timestep" => Timestep(f64)),
    ("isInitialVelocityAdded" => IsInitialVelocityAdded(bool)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InitialVelocityCoordinates {
    #[serde(rename = "INITIAL_VELOCITY_IN_WORLD_COORDINATES")]
    InitialVelocityInWorldCoordinates = 0,
    #[serde(rename = "INITIAL_VELOCITY_IN_MODEL_COORDINATES")]
    InitialVelocityInModelCoordinates = 1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MotionMode {
    #[serde(rename = "MOTION_MODE_FOLLOW_ANIMATION")]
    MotionModeFollowAnimation = 0,
    #[serde(rename = "MOTION_MODE_DYNAMIC")]
    MotionModeDynamic = 1,
}
