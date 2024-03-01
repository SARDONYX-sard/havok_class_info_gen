//! A Rust structure that implements a serializer/deserializer corresponding to `hkpCharacterRigidBodyCinfo`, a class defined in C++
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
/// -    size: 112
/// -  vtable: true
/// -  parent: hkpCharacterControllerCinfo/`da8c7d7d`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpCharacterRigidBodyCinfo<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpCharacterRigidBodyCinfo"`: Name of this class.
    #[serde(default = "HkpCharacterRigidBodyCinfo::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x892f441`: Unique value of this class.
    #[serde(default = "HkpCharacterRigidBodyCinfo::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpCharacterRigidBodyCinfoHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpCharacterRigidBodyCinfoHkParam<'a>>
}

impl HkpCharacterRigidBodyCinfo<'_> {
    /// Return `"hkpCharacterRigidBodyCinfo"`, which is the name of this class.
    ///
    /// # NOTE
    /// It is the name of the Rust structure, not the original class name in C++.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "HkpCharacterRigidBodyCinfo".into()
    }

    /// Return `"0x892f441"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x892f441".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpCharacterRigidBodyCinfoHkParam<'a> {
    /// # Information on fields in the original C++ class
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(u32),
    /// # Information on fields in the original C++ class
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shape")]
    Shape(Box<HkpShape>),
    /// # Information on fields in the original C++ class
    /// -   name:`"position"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "position")]
    Position(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"rotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotation")]
    Rotation(cgmath::Quaternion<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"mass"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mass")]
    Mass(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"friction"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "friction")]
    Friction(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxLinearVelocity"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxLinearVelocity")]
    MaxLinearVelocity(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"allowedPenetrationDepth"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allowedPenetrationDepth")]
    AllowedPenetrationDepth(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"up"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "up")]
    Up(cgmath::Vector4<f32>),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxSlope"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSlope")]
    MaxSlope(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxForce"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxForce")]
    MaxForce(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"unweldingHeightOffsetFactor"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "unweldingHeightOffsetFactor")]
    UnweldingHeightOffsetFactor(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"maxSpeedForSimplexSolver"`
    /// -   type: `hkReal`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSpeedForSimplexSolver")]
    MaxSpeedForSimplexSolver(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"supportDistance"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "supportDistance")]
    SupportDistance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"hardSupportDistance"`
    /// -   type: `hkReal`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hardSupportDistance")]
    HardSupportDistance(f64),
    /// # Information on fields in the original C++ class
    /// -   name:`"vdbColor"`
    /// -   type: `hkInt32`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vdbColor")]
    VdbColor(i32),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpCharacterRigidBodyCinfoHkParam<'de>, "@name",
    ("collisionFilterInfo" => CollisionFilterInfo(u32)),
    ("shape" => Shape(Box<HkpShape>)),
    ("position" => Position(cgmath::Vector4<f32>)),
    ("rotation" => Rotation(cgmath::Quaternion<f32>)),
    ("mass" => Mass(f64)),
    ("friction" => Friction(f64)),
    ("maxLinearVelocity" => MaxLinearVelocity(f64)),
    ("allowedPenetrationDepth" => AllowedPenetrationDepth(f64)),
    ("up" => Up(cgmath::Vector4<f32>)),
    ("maxSlope" => MaxSlope(f64)),
    ("maxForce" => MaxForce(f64)),
    ("unweldingHeightOffsetFactor" => UnweldingHeightOffsetFactor(f64)),
    ("maxSpeedForSimplexSolver" => MaxSpeedForSimplexSolver(f64)),
    ("supportDistance" => SupportDistance(f64)),
    ("hardSupportDistance" => HardSupportDistance(f64)),
    ("vdbColor" => VdbColor(i32)),
}
