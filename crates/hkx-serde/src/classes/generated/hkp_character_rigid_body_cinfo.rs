//! A Rust structure that implements a serializer/deserializer corresponding to `hkpCharacterRigidBodyCinfo`, a class defined in C++
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

    /// `"hkpCharacterRigidBodyCinfo"`: The original C++ class name.
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
    /// Return `"hkpCharacterRigidBodyCinfo"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpCharacterRigidBodyCinfo".into()
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
    /// # Field information in the original C++ class
    /// -   name:`"collisionFilterInfo"`
    /// -   type: `hkUint32`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "collisionFilterInfo")]
    CollisionFilterInfo(Primitive<u32>),
    /// # Field information in the original C++ class
    /// -   name:`"shape"`
    /// -   type: `struct hkpShape*`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "shape")]
    Shape(Cow<'a, str>),
    /// # Field information in the original C++ class
    /// -   name:`"position"`
    /// -   type: `hkVector4`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "position")]
    Position(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"rotation"`
    /// -   type: `hkQuaternion`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "rotation")]
    Rotation(Quaternion<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"mass"`
    /// -   type: `hkReal`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "mass")]
    Mass(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"friction"`
    /// -   type: `hkReal`
    /// - offset: 52
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "friction")]
    Friction(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxLinearVelocity"`
    /// -   type: `hkReal`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxLinearVelocity")]
    MaxLinearVelocity(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"allowedPenetrationDepth"`
    /// -   type: `hkReal`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "allowedPenetrationDepth")]
    AllowedPenetrationDepth(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"up"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "up")]
    Up(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxSlope"`
    /// -   type: `hkReal`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSlope")]
    MaxSlope(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxForce"`
    /// -   type: `hkReal`
    /// - offset: 84
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxForce")]
    MaxForce(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"unweldingHeightOffsetFactor"`
    /// -   type: `hkReal`
    /// - offset: 88
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "unweldingHeightOffsetFactor")]
    UnweldingHeightOffsetFactor(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxSpeedForSimplexSolver"`
    /// -   type: `hkReal`
    /// - offset: 92
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxSpeedForSimplexSolver")]
    MaxSpeedForSimplexSolver(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"supportDistance"`
    /// -   type: `hkReal`
    /// - offset: 96
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "supportDistance")]
    SupportDistance(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"hardSupportDistance"`
    /// -   type: `hkReal`
    /// - offset: 100
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "hardSupportDistance")]
    HardSupportDistance(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"vdbColor"`
    /// -   type: `hkInt32`
    /// - offset: 104
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "vdbColor")]
    VdbColor(Primitive<i32>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpCharacterRigidBodyCinfoHkParam<'de>, "@name",
    ("collisionFilterInfo" => CollisionFilterInfo(Primitive<u32>)),
    ("shape" => Shape(Cow<'a, str>)),
    ("position" => Position(Vector4<f32>)),
    ("rotation" => Rotation(Quaternion<f32>)),
    ("mass" => Mass(Primitive<f32>)),
    ("friction" => Friction(Primitive<f32>)),
    ("maxLinearVelocity" => MaxLinearVelocity(Primitive<f32>)),
    ("allowedPenetrationDepth" => AllowedPenetrationDepth(Primitive<f32>)),
    ("up" => Up(Vector4<f32>)),
    ("maxSlope" => MaxSlope(Primitive<f32>)),
    ("maxForce" => MaxForce(Primitive<f32>)),
    ("unweldingHeightOffsetFactor" => UnweldingHeightOffsetFactor(Primitive<f32>)),
    ("maxSpeedForSimplexSolver" => MaxSpeedForSimplexSolver(Primitive<f32>)),
    ("supportDistance" => SupportDistance(Primitive<f32>)),
    ("hardSupportDistance" => HardSupportDistance(Primitive<f32>)),
    ("vdbColor" => VdbColor(Primitive<i32>)),
}
