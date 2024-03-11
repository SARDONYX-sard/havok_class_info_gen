//! A Rust structure that implements a serializer/deserializer corresponding to `hkpBallGun`, a class defined in C++
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
/// -    size: 96
/// -  vtable: true
/// -  parent: hkpFirstPersonGun/`852ab70b`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpBallGun<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpBallGun"`: The original C++ class name.
    #[serde(default = "HkpBallGun::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x57b06d35`: Unique value of this class.
    #[serde(default = "HkpBallGun::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpBallGunHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpBallGunHkParam<'a>>
}

impl HkpBallGun<'_> {
    /// Return `"hkpBallGun"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpBallGun".into()
    }

    /// Return `"0x57b06d35"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x57b06d35".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpBallGunHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"bulletRadius"`
    /// -   type: `hkReal`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bulletRadius")]
    BulletRadius(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"bulletVelocity"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bulletVelocity")]
    BulletVelocity(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"bulletMass"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bulletMass")]
    BulletMass(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"damageMultiplier"`
    /// -   type: `hkReal`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "damageMultiplier")]
    DamageMultiplier(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"maxBulletsInWorld"`
    /// -   type: `hkInt32`
    /// - offset: 48
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxBulletsInWorld")]
    MaxBulletsInWorld(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"bulletOffsetFromCenter"`
    /// -   type: `hkVector4`
    /// - offset: 64
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "bulletOffsetFromCenter")]
    BulletOffsetFromCenter(Vector4<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"addedBodies"`
    /// -   type: `void*`
    /// - offset: 80
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "addedBodies", skip_serializing)]
    AddedBodies(()),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpBallGunHkParam<'de>, "@name",
    ("bulletRadius" => BulletRadius(Primitive<f32>)),
    ("bulletVelocity" => BulletVelocity(Primitive<f32>)),
    ("bulletMass" => BulletMass(Primitive<f32>)),
    ("damageMultiplier" => DamageMultiplier(Primitive<f32>)),
    ("maxBulletsInWorld" => MaxBulletsInWorld(Primitive<i32>)),
    ("bulletOffsetFromCenter" => BulletOffsetFromCenter(Vector4<f32>)),
    ("addedBodies" => AddedBodies(())),
}
