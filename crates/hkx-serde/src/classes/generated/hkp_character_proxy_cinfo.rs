//! A Rust structure that implements a serializer/deserializer corresponding to `hkpCharacterProxyCinfo`, a class defined in C++
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
/// -  parent: hkpCharacterControllerCinfo/`da8c7d7d`(Non prefix hex signature)
/// - version: 1
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpCharacterProxyCinfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpCharacterProxyCinfo"`: Name of this class.
    #[serde(default = "HkpCharacterProxyCinfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x586d97b2`: Unique value of this class.
    #[serde(default = "HkpCharacterProxyCinfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpCharacterProxyCinfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpCharacterProxyCinfoHkParam<'a>>
}

impl HkpCharacterProxyCinfo<'_> {
    /// Return `"hkpCharacterProxyCinfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpCharacterProxyCinfo".into()
    }

    /// Return `"0x586d97b2"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x586d97b2".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCharacterProxyCinfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"position"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "position")]
    Position(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"velocity"`
    /// -   type: `hkVector4`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "velocity")]
    Velocity(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"dynamicFriction"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "dynamicFriction")]
    DynamicFriction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"staticFriction"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "staticFriction")]
    StaticFriction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"keepContactTolerance"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keepContactTolerance")]
    KeepContactTolerance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"up"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "up")]
    Up(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"extraUpStaticFriction"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extraUpStaticFriction")]
    ExtraUpStaticFriction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"extraDownStaticFriction"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "extraDownStaticFriction")]
    ExtraDownStaticFriction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"shapePhantom"`
    /// -   type: `struct hkpShapePhantom*`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shapePhantom")]
    ShapePhantom(Box<HkpShapePhantom>),
    /// # Information on fields in the original C++ class
    /// -   name:`"keepDistance"`
    /// -   type: `hkReal`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keepDistance")]
    KeepDistance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"contactAngleSensitivity"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "contactAngleSensitivity")]
    ContactAngleSensitivity(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"userPlanes"`
    /// -   type: `hkUint32`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "userPlanes")]
    UserPlanes(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxCharacterSpeedForSolver"`
    /// -   type: `hkReal`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxCharacterSpeedForSolver")]
    MaxCharacterSpeedForSolver(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"characterStrength"`
    /// -   type: `hkReal`
    /// - offset: 108
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterStrength")]
    CharacterStrength(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"characterMass"`
    /// -   type: `hkReal`
    /// - offset: 112
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "characterMass")]
    CharacterMass(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxSlope"`
    /// -   type: `hkReal`
    /// - offset: 116
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSlope")]
    MaxSlope(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"penetrationRecoverySpeed"`
    /// -   type: `hkReal`
    /// - offset: 120
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "penetrationRecoverySpeed")]
    PenetrationRecoverySpeed(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxCastIterations"`
    /// -   type: `hkInt32`
    /// - offset: 124
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxCastIterations")]
    MaxCastIterations(i32),
    /// # Information on fields in the original C++ class
    /// -   name:`"refreshManifoldInCheckSupport"`
    /// -   type: `hkBool`
    /// - offset: 128
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "refreshManifoldInCheckSupport")]
    RefreshManifoldInCheckSupport(bool),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpCharacterProxyCinfoHkParam<'de>, "@name",
    ("position" => Position(cgmath::Vector4<f32>)),
    ("velocity" => Velocity(cgmath::Vector4<f32>)),
    ("dynamicFriction" => DynamicFriction(f64)),
    ("staticFriction" => StaticFriction(f64)),
    ("keepContactTolerance" => KeepContactTolerance(f64)),
    ("up" => Up(cgmath::Vector4<f32>)),
    ("extraUpStaticFriction" => ExtraUpStaticFriction(f64)),
    ("extraDownStaticFriction" => ExtraDownStaticFriction(f64)),
    ("shapePhantom" => ShapePhantom(Box<HkpShapePhantom>)),
    ("keepDistance" => KeepDistance(f64)),
    ("contactAngleSensitivity" => ContactAngleSensitivity(f64)),
    ("userPlanes" => UserPlanes(u32)),
    ("maxCharacterSpeedForSolver" => MaxCharacterSpeedForSolver(f64)),
    ("characterStrength" => CharacterStrength(f64)),
    ("characterMass" => CharacterMass(f64)),
    ("maxSlope" => MaxSlope(f64)),
    ("penetrationRecoverySpeed" => PenetrationRecoverySpeed(f64)),
    ("maxCastIterations" => MaxCastIterations(i32)),
    ("refreshManifoldInCheckSupport" => RefreshManifoldInCheckSupport(bool)),
}
