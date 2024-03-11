//! A Rust structure that implements a serializer/deserializer corresponding to `hkpProjectileGun`, a class defined in C++
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
/// -    size: 64
/// -  vtable: true
/// -  parent: hkpFirstPersonGun/`852ab70b`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpProjectileGun<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpProjectileGun"`: The original C++ class name.
    #[serde(default = "HkpProjectileGun::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0xb4f30148`: Unique value of this class.
    #[serde(default = "HkpProjectileGun::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpProjectileGunHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpProjectileGunHkParam<'a>>
}

impl HkpProjectileGun<'_> {
    /// Return `"hkpProjectileGun"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpProjectileGun".into()
    }

    /// Return `"0xb4f30148"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0xb4f30148".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpProjectileGunHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"maxProjectiles"`
    /// -   type: `hkInt32`
    /// - offset: 32
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "maxProjectiles")]
    MaxProjectiles(Primitive<i32>),
    /// # Field information in the original C++ class
    /// -   name:`"reloadTime"`
    /// -   type: `hkReal`
    /// - offset: 36
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "reloadTime")]
    ReloadTime(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"reload"`
    /// -   type: `hkReal`
    /// - offset: 40
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "reload", skip_serializing)]
    Reload(Primitive<f32>),
    /// # Field information in the original C++ class
    /// -   name:`"projectiles"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 44
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "projectiles", skip_serializing)]
    Projectiles(Vec<()>),
    /// # Field information in the original C++ class
    /// -   name:`"world"`
    /// -   type: `void*`
    /// - offset: 56
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "world", skip_serializing)]
    World(()),
    /// # Field information in the original C++ class
    /// -   name:`"destructionWorld"`
    /// -   type: `void*`
    /// - offset: 60
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "destructionWorld", skip_serializing)]
    DestructionWorld(()),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpProjectileGunHkParam<'de>, "@name",
    ("maxProjectiles" => MaxProjectiles(Primitive<i32>)),
    ("reloadTime" => ReloadTime(Primitive<f32>)),
    ("reload" => Reload(Primitive<f32>)),
    ("projectiles" => Projectiles(Vec<()>)),
    ("world" => World(())),
    ("destructionWorld" => DestructionWorld(())),
}
