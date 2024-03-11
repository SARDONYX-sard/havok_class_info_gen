//! A Rust structure that implements a serializer/deserializer corresponding to `hkpFirstPersonGun`, a class defined in C++
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
/// -    size: 32
/// -  vtable: true
/// -  parent: hkReferencedObject/`3b1c1113`(Non prefix hex signature)
/// - version: 0
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hkobject")]
pub struct HkpFirstPersonGun<'a> {
    /// e.g. `#0106`
    ///
    /// These names are referenced (in C++ implementations) by vectors that store pointers to a structure and a class.
    #[serde(rename = "@name", borrow)]
    pub name: Cow<'a, str>,

    /// `"hkpFirstPersonGun"`: The original C++ class name.
    #[serde(default = "HkpFirstPersonGun::class_name")]
    #[serde(rename = "@class", borrow)]
    pub class: Cow<'a, str>,

    /// `0x852ab70b`: Unique value of this class.
    #[serde(default = "HkpFirstPersonGun::signature")]
    #[serde(rename = "@signature", borrow)]
    pub signature: Cow<'a, str>,

    /// The `"hkparam"` tag (C++ field) vector
    #[serde(bound(deserialize = "Vec<HkpFirstPersonGunHkParam<'a>>: Deserialize<'de>"))]
    #[serde(rename = "hkparam")]
    pub hkparams: Vec<HkpFirstPersonGunHkParam<'a>>
}

impl HkpFirstPersonGun<'_> {
    /// Return `"hkpFirstPersonGun"`, which is the name of this C++ class.
    ///
    /// # NOTE
    /// It is not the name of the Rust structure.
    #[inline]
    pub fn class_name() -> Cow<'static, str> {
        "hkpFirstPersonGun".into()
    }

    /// Return `"0x852ab70b"`, which is the signature of this class.
    #[inline]
    pub fn signature() -> Cow<'static, str> {
        "0x852ab70b".into()
    }
}

/// In XML, the value of the `name` attribute of the `hkparam` tag.
///
/// In C++, it represents the name of one field in the class.
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "@name")]
pub enum HkpFirstPersonGunHkParam<'a> {
    /// # Field information in the original C++ class
    /// -   name:`"type"`
    /// -   type: `enum unknown`
    /// - offset: 8
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "type", skip_serializing)]
    Type(Unknown),
    /// # Field information in the original C++ class
    /// -   name:`"name"`
    /// -   type: `hkStringPtr`
    /// - offset: 12
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "name")]
    Name(Primitive<Cow<'a, str>>),
    /// # Field information in the original C++ class
    /// -   name:`"keyboardKey"`
    /// -   type: `enum KeyboardKey`
    /// - offset: 16
    /// -  flags: `FLAGS_NONE`
    #[serde(rename = "keyboardKey")]
    KeyboardKey(KeyboardKey),
    /// # Field information in the original C++ class
    /// -   name:`"listeners"`
    /// -   type: `hkArray&lt;void*&gt;`
    /// - offset: 20
    /// -  flags: `FLAGS_NONE | SERIALIZE_IGNORED`
    #[serde(rename = "listeners", skip_serializing)]
    Listeners(Vec<()>),
}

// Implementing a deserializer for enum manually with macros is necessary
// because the type needs to change depending on the value of the `"name"` attribute in the XML.
impl_deserialize_for_internally_tagged_enum! {
    HkpFirstPersonGunHkParam<'de>, "@name",
    ("type" => Type(Unknown)),
    ("name" => Name(Primitive<Cow<'a, str>>)),
    ("keyboardKey" => KeyboardKey(KeyboardKey)),
    ("listeners" => Listeners(Vec<()>)),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "WEAPON_TYPE_INVALID")]
    WeaponTypeInvalid = 0,
    #[serde(rename = "WEAPON_TYPE_BALLGUN")]
    WeaponTypeBallgun = 1,
    #[serde(rename = "WEAPON_TYPE_GRENADEGUN")]
    WeaponTypeGrenadegun = 2,
    #[serde(rename = "WEAPON_TYPE_GRAVITYGUN")]
    WeaponTypeGravitygun = 3,
    #[serde(rename = "WEAPON_TYPE_MOUNTEDBALLGUN")]
    WeaponTypeMountedballgun = 4,
    #[serde(rename = "WEAPON_TYPE_TWEAKERGUN")]
    WeaponTypeTweakergun = 5,
    #[serde(rename = "WEAPON_TYPE_MISSILEGUN")]
    WeaponTypeMissilegun = 6,
    #[serde(rename = "WEAPON_TYPE_RAYCASTGUN")]
    WeaponTypeRaycastgun = 7,
    #[serde(rename = "WEAPON_TYPE_SPHEREGUN")]
    WeaponTypeSpheregun = 8,
    #[serde(rename = "WEAPON_TYPE_STICKYGUN")]
    WeaponTypeStickygun = 9,
    #[serde(rename = "WEAPON_TYPE_NUM_TYPES")]
    WeaponTypeNumTypes = 10,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum KeyboardKey {
    #[serde(rename = "KEY_F1")]
    KeyF1 = 112,
    #[serde(rename = "KEY_F2")]
    KeyF2 = 113,
    #[serde(rename = "KEY_F3")]
    KeyF3 = 114,
    #[serde(rename = "KEY_F4")]
    KeyF4 = 115,
    #[serde(rename = "KEY_F5")]
    KeyF5 = 116,
    #[serde(rename = "KEY_F6")]
    KeyF6 = 117,
    #[serde(rename = "KEY_F7")]
    KeyF7 = 118,
    #[serde(rename = "KEY_F8")]
    KeyF8 = 119,
    #[serde(rename = "KEY_F9")]
    KeyF9 = 120,
    #[serde(rename = "KEY_F10")]
    KeyF10 = 121,
    #[serde(rename = "KEY_F11")]
    KeyF11 = 122,
    #[serde(rename = "KEY_F12")]
    KeyF12 = 123,
}
