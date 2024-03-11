//! A Rust structure that implements a serializer/deserializer corresponding to `hkbProxyModifierProxyInfo`, a class defined in C++
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
/// -    size: 80
/// -  vtable: false
/// -  parent: None/`0`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkbProxyModifierProxyInfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkbProxyModifierProxyInfo"`: The original C++ class name.
    #[serde(default = "HkbProxyModifierProxyInfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x39de637e`: Unique value of this class.
    #[serde(default = "HkbProxyModifierProxyInfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkbProxyModifierProxyInfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkbProxyModifierProxyInfoHkParam<'a>>
}

impl HkbProxyModifierProxyInfo<'_> {
    /// Return `"hkbProxyModifierProxyInfo"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkbProxyModifierProxyInfo".into()
    }

    /// Return `"0x39de637e"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x39de637e".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkbProxyModifierProxyInfoHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"dynamicFriction"`
    /// -   type: `hkReal`
    /// - offset: 0
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dynamicFriction")]
    DynamicFriction(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"staticFriction"`
    /// -   type: `hkReal`
    /// - offset: 4
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "staticFriction")]
    StaticFriction(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"keepContactTolerance"`
    /// -   type: `hkReal`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keepContactTolerance")]
    KeepContactTolerance(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"up"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "up")]
    Up(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"keepDistance"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keepDistance")]
    KeepDistance(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"contactAngleSensitivity"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactAngleSensitivity")]
    ContactAngleSensitivity(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"userPlanes"`
    /// -   type: `hkUint32`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userPlanes")]
    UserPlanes(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxCharacterSpeedForSolver"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxCharacterSpeedForSolver")]
    MaxCharacterSpeedForSolver(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"characterStrength"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterStrength")]
    CharacterStrength(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"characterMass"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterMass")]
    CharacterMass(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxSlope"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSlope")]
    MaxSlope(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"penetrationRecoverySpeed"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "penetrationRecoverySpeed")]
    PenetrationRecoverySpeed(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxCastIterations"`
    /// -   type: `hkInt32`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxCastIterations")]
    MaxCastIterations(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"refreshManifoldInCheckSupport"`
    /// -   type: `hkBool`
    /// - offset: 68
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "refreshManifoldInCheckSupport")]
    RefreshManifoldInCheckSupport(Primitive<bool>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkbProxyModifierProxyInfoHkParam<'de>, "@name",
    ("dynamicFriction" => DynamicFriction(Primitive<f32>)),
    ("staticFriction" => StaticFriction(Primitive<f32>)),
    ("keepContactTolerance" => KeepContactTolerance(Primitive<f32>)),
    ("up" => Up(Vector4<f32>)),
    ("keepDistance" => KeepDistance(Primitive<f32>)),
    ("contactAngleSensitivity" => ContactAngleSensitivity(Primitive<f32>)),
    ("userPlanes" => UserPlanes(Primitive<u32>)),
    ("maxCharacterSpeedForSolver" => MaxCharacterSpeedForSolver(Primitive<f32>)),
    ("characterStrength" => CharacterStrength(Primitive<f32>)),
    ("characterMass" => CharacterMass(Primitive<f32>)),
    ("maxSlope" => MaxSlope(Primitive<f32>)),
    ("penetrationRecoverySpeed" => PenetrationRecoverySpeed(Primitive<f32>)),
    ("maxCastIterations" => MaxCastIterations(Primitive<i32>)),
    ("refreshManifoldInCheckSupport" => RefreshManifoldInCheckSupport(Primitive<bool>)),
}
